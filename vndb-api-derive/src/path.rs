use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::sync::atomic::{AtomicUsize, Ordering};
use syn::Visibility;

use crate::selection::Selection;

static HIDDEN_MACRO_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy)]
pub struct PathConfig {
    pub derive_name: &'static str,
}

pub fn helpers(
    enum_ident: &Ident,
    enum_vis: &Visibility,
    selections: &[Selection],
    config: PathConfig,
) -> TokenStream {
    let filter_helpers = filter_helpers(selections, config);
    let filter_constructors = filter_constructors(selections, config);
    let selector_macro = selector_macro(enum_ident, enum_vis, selections, config);

    quote! {
        impl #enum_ident {
            #filter_helpers
            #filter_constructors
        }

        #selector_macro
    }
}

fn wrapped_nested_item(selection: &Selection) -> TokenStream {
    if selection.nested_boxed {
        quote! { std::boxed::Box::new(item) }
    } else {
        quote! { item }
    }
}

fn selector_macro(
    enum_ident: &Ident,
    enum_vis: &Visibility,
    selections: &[Selection],
    config: PathConfig,
) -> TokenStream {
    if config.derive_name == "VndbFiltersEnum" {
        filter_macro(enum_ident, enum_vis, selections, config)
    } else {
        field_sort_selector_macro(enum_ident, enum_vis, selections, config)
    }
}

fn field_sort_selector_macro(
    enum_ident: &Ident,
    enum_vis: &Visibility,
    selections: &[Selection],
    config: PathConfig,
) -> TokenStream {
    let hidden_macro_ident = hidden_macro_ident("selector", enum_ident);
    let push_arms = field_sort_selector_push_arms(enum_ident, selections, config);
    let root_group_arm = field_selector_root_group_arm(&hidden_macro_ident, config);
    let error_message = format!("unknown VNDB selector field for {}", enum_ident);

    let macro_rules = quote! {
        macro_rules! #hidden_macro_ident {
            (@parse $items:ident [] ) => {
                compile_error!("selector must contain at least one field")
            };

            (@parse $items:ident [] , $($rest:tt)*) => {
                compile_error!("empty selector item")
            };

            (@parse $items:ident [$($item:tt)+] ,) => {
                compile_error!("trailing comma in selector")
            };

            (@parse $items:ident [$($item:tt)+] , $($rest:tt)+) => {{
                #hidden_macro_ident!(@push $items; $($item)+);
                #hidden_macro_ident!(@parse $items [] $($rest)+);
            }};

            (@parse $items:ident [$($item:tt)+]) => {
                #hidden_macro_ident!(@push $items; $($item)+);
            };

            (@parse $items:ident [$($item:tt)*] $next:tt $($rest:tt)*) => {
                #hidden_macro_ident!(@parse $items [$($item)* $next] $($rest)*);
            };

            (@push $items:ident; {}) => {
                compile_error!("empty selector group")
            };

            #root_group_arm
            #( #push_arms )*

            (@push $items:ident; $($tokens:tt)+) => {
                compile_error!(#error_message)
            };

            () => {
                compile_error!("selector must contain at least one field")
            };

            ($($tokens:tt)+) => {{
                let mut items = Vec::new();
                #hidden_macro_ident!(@parse items [] $($tokens)+);
                items
            }};
        }
    };

    if matches!(enum_vis, Visibility::Public(_)) {
        quote! {
            #[doc(hidden)]
            #[macro_export]
            #macro_rules

            #[doc(hidden)]
            #[allow(unused_imports)]
            pub use #hidden_macro_ident as #enum_ident;
        }
    } else {
        quote! {
            #macro_rules

            #[doc(hidden)]
            #[allow(unused_imports)]
            pub(crate) use #hidden_macro_ident as #enum_ident;
        }
    }
}

fn field_selector_root_group_arm(hidden_macro_ident: &Ident, config: PathConfig) -> TokenStream {
    if config.derive_name == "VndbFieldsEnum" {
        quote! {
            (@push $items:ident; { $($inner:tt)+ }) => {
                #hidden_macro_ident!(@parse $items [] $($inner)+);
            };
        }
    } else {
        quote! {}
    }
}

fn field_sort_selector_push_arms(
    enum_ident: &Ident,
    selections: &[Selection],
    config: PathConfig,
) -> Vec<TokenStream> {
    let non_flattened = selections
        .iter()
        .filter(|selection| !selection.flattened)
        .map(|selection| field_sort_selector_push_arm(enum_ident, selection, config));
    let flattened = selections
        .iter()
        .filter(|selection| selection.flattened)
        .map(|selection| flattened_field_selector_push_arm(enum_ident, selection));

    non_flattened.chain(flattened).collect()
}

fn field_sort_selector_push_arm(
    enum_ident: &Ident,
    selection: &Selection,
    config: PathConfig,
) -> TokenStream {
    let variant_ident = &selection.variant_ident;
    let path_ident = filter_path_ident(&selection.path, variant_ident.span());

    match (&selection.nested_enum_ident, config.derive_name) {
        (Some(nested_enum_ident), "VndbFieldsEnum") => {
            let wrap_item = wrapped_nested_item(selection);

            quote! {
                (@push $items:ident; #path_ident . {}) => {
                    compile_error!("empty selector group")
                };

                (@push $items:ident; #path_ident {}) => {
                    compile_error!("empty selector group")
                };

                (@push $items:ident; #path_ident . { $($inner:tt)+ }) => {
                    for item in #nested_enum_ident!($($inner)+) {
                        $items.push(#enum_ident::#variant_ident(#wrap_item));
                    }
                };

                (@push $items:ident; #path_ident { $($inner:tt)+ }) => {
                    for item in #nested_enum_ident!($($inner)+) {
                        $items.push(#enum_ident::#variant_ident(#wrap_item));
                    }
                };

                (@push $items:ident; #path_ident . $($tail:tt)+) => {
                    for item in #nested_enum_ident!($($tail)+) {
                        $items.push(#enum_ident::#variant_ident(#wrap_item));
                    }
                };
            }
        }
        _ => quote! {
            (@push $items:ident; #path_ident) => {
                $items.push(#enum_ident::#variant_ident);
            };
        },
    }
}

fn flattened_field_selector_push_arm(enum_ident: &Ident, selection: &Selection) -> TokenStream {
    let variant_ident = &selection.variant_ident;
    let nested_enum_ident = selection
        .nested_enum_ident
        .as_ref()
        .expect("flattened selections must have a child enum");
    let wrap_item = wrapped_nested_item(selection);

    quote! {
        (@push $items:ident; $($tokens:tt)+) => {
            for item in #nested_enum_ident!($($tokens)+) {
                $items.push(#enum_ident::#variant_ident(#wrap_item));
            }
        };
    }
}

fn filter_helpers(selections: &[Selection], config: PathConfig) -> TokenStream {
    if config.derive_name != "VndbFiltersEnum" {
        return quote! {};
    }

    let predicate_arms = selections
        .iter()
        .map(filter_predicate_arm)
        .collect::<Vec<_>>();

    quote! {
        #[doc(hidden)]
        pub fn __vndb_filter_predicate(
            &self,
            operator: &str,
            value: serde_json::Value,
        ) -> serde_json::Value {
            match self {
                #( #predicate_arms, )*
            }
        }
    }
}

fn filter_constructors(selections: &[Selection], config: PathConfig) -> TokenStream {
    if config.derive_name != "VndbFiltersEnum" {
        return quote! {};
    }

    let constructors = selections
        .iter()
        .filter(|selection| !selection.flattened)
        .map(filter_constructor)
        .chain(
            selections
                .iter()
                .filter(|selection| selection.flattened)
                .map(filter_constructor),
        )
        .collect::<Vec<_>>();

    quote! {
        #( #constructors )*
    }
}

fn filter_constructor(selection: &Selection) -> TokenStream {
    let variant_ident = &selection.variant_ident;
    let method_ident = filter_method_ident(&selection.path, variant_ident.span());

    if selection.flattened {
        let nested_enum_ident = selection
            .nested_enum_ident
            .as_ref()
            .expect("flattened selections must have a child enum");
        let wrap_item = wrapped_nested_item(selection);

        return quote! {
            #[doc(hidden)]
            pub fn #method_ident<V, Ops, Nullability>(
                child: ::vndb_api_macros_support::filter::FilterField<#nested_enum_ident, V, Ops, Nullability>,
            ) -> ::vndb_api_macros_support::filter::FilterField<Self, V, Ops, Nullability>
            where
                V: ::vndb_api_macros_support::filter::VndbFilterValueType,
            {
                let item = child.__into_field();

                ::vndb_api_macros_support::filter::FilterField::__new(
                    Self::#variant_ident(#wrap_item),
                    Self::__vndb_filter_predicate,
                )
            }
        };
    }

    match &selection.nested_enum_ident {
        Some(nested_enum_ident) => {
            let path = &selection.path;
            let wrap_item = wrapped_nested_item(selection);
            let nested_method_ident =
                filter_nested_method_ident(&selection.path, variant_ident.span());

            quote! {
                pub fn #method_ident() -> ::vndb_api_macros_support::filter::NestedFilterField<Self, #nested_enum_ident> {
                    ::vndb_api_macros_support::filter::NestedFilterField::__new(#path)
                }

                #[doc(hidden)]
                pub fn #nested_method_ident<V, Ops, Nullability>(
                    child: ::vndb_api_macros_support::filter::FilterField<#nested_enum_ident, V, Ops, Nullability>,
                ) -> ::vndb_api_macros_support::filter::FilterField<Self, V, Ops, Nullability>
                where
                    V: ::vndb_api_macros_support::filter::VndbFilterValueType,
                {
                    let item = child.__into_field();

                    ::vndb_api_macros_support::filter::FilterField::__new(
                        Self::#variant_ident(#wrap_item),
                        Self::__vndb_filter_predicate,
                    )
                }
            }
        }
        None => {
            let target = selection.filter_target();
            let value_ty = target
                .value_ty
                .as_ref()
                .expect("leaf filters must have a value type");
            let ops_marker = filter_ops_marker(target.ops.bits());
            let nullability_marker = filter_nullability_marker(target.nullable);

            quote! {
                pub fn #method_ident() -> ::vndb_api_macros_support::filter::FilterField<
                    Self,
                    #value_ty,
                    #ops_marker,
                    #nullability_marker,
                > {
                    ::vndb_api_macros_support::filter::FilterField::__new(
                        Self::#variant_ident,
                        Self::__vndb_filter_predicate,
                    )
                }
            }
        }
    }
}

fn filter_macro(
    enum_ident: &Ident,
    enum_vis: &Visibility,
    selections: &[Selection],
    config: PathConfig,
) -> TokenStream {
    if config.derive_name != "VndbFiltersEnum" {
        return quote! {};
    }

    let hidden_macro_ident = hidden_macro_ident("filter", enum_ident);
    let arms = filter_macro_arms(enum_ident, selections);
    let error_message = format!("unknown VNDB filter field for {}", enum_ident);

    if matches!(enum_vis, Visibility::Public(_)) {
        quote! {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! #hidden_macro_ident {
                #( #arms )*
                ($($tokens:tt)+) => {
                    compile_error!(#error_message)
                };
            }

            #[doc(hidden)]
            #[allow(unused_imports)]
            pub use #hidden_macro_ident as #enum_ident;
        }
    } else {
        quote! {
            macro_rules! #hidden_macro_ident {
                #( #arms )*
                ($($tokens:tt)+) => {
                    compile_error!(#error_message)
                };
            }

            #[doc(hidden)]
            #[allow(unused_imports)]
            pub(crate) use #hidden_macro_ident as #enum_ident;
        }
    }
}

fn hidden_macro_ident(kind: &str, enum_ident: &Ident) -> Ident {
    let id = HIDDEN_MACRO_ID.fetch_add(1, Ordering::Relaxed);
    format_ident!("__vndb_{}_macro_{}_{}", kind, enum_ident, id)
}

fn filter_macro_arms(enum_ident: &Ident, selections: &[Selection]) -> Vec<TokenStream> {
    selections
        .iter()
        .filter(|selection| !selection.flattened)
        .map(|selection| {
            let method_ident = filter_method_ident(&selection.path, selection.variant_ident.span());
            let path_ident = filter_path_ident(&selection.path, selection.variant_ident.span());

            match &selection.nested_enum_ident {
                Some(nested_enum_ident) => {
                    let nested_method_ident =
                        filter_nested_method_ident(&selection.path, selection.variant_ident.span());

                    quote! {
                        (#path_ident) => {
                            #enum_ident::#method_ident()
                        };

                        (#path_ident . $($tail:tt)+) => {
                            #enum_ident::#nested_method_ident(#nested_enum_ident!($($tail)+))
                        };
                    }
                }
                None => quote! {
                    (#path_ident) => {
                        #enum_ident::#method_ident()
                    };
                },
            }
        })
        .collect()
}

fn filter_method_ident(path: &str, span: Span) -> Ident {
    filter_path_ident(path, span)
}

fn filter_nested_method_ident(path: &str, span: Span) -> Ident {
    format_ident!("__vndb_filter_{}", path, span = span)
}

fn filter_path_ident(path: &str, span: Span) -> Ident {
    if is_rust_keyword(path) {
        Ident::new_raw(path, span)
    } else {
        Ident::new(path, span)
    }
}

fn is_rust_keyword(ident: &str) -> bool {
    matches!(
        ident,
        "as" | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

fn filter_ops_marker(op_bits: u8) -> TokenStream {
    if op_bits == crate::selection::FilterOperatorSet::Ordering.bits() {
        quote! { ::vndb_api_macros_support::filter::OrderedOps }
    } else {
        quote! { ::vndb_api_macros_support::filter::EqualityOps }
    }
}

fn filter_nullability_marker(nullable: bool) -> TokenStream {
    if nullable {
        quote! { ::vndb_api_macros_support::filter::Nullable }
    } else {
        quote! { ::vndb_api_macros_support::filter::NonNullable }
    }
}

fn filter_predicate_arm(selection: &Selection) -> TokenStream {
    let variant_ident = &selection.variant_ident;
    let path = &selection.path;

    if selection.flattened {
        return quote! {
            Self::#variant_ident(item) => item.__vndb_filter_predicate(operator, value)
        };
    }

    match &selection.nested_enum_ident {
        Some(_) => quote! {
            Self::#variant_ident(item) => serde_json::json!([
                #path,
                "=",
                item.__vndb_filter_predicate(operator, value)
            ])
        },
        None => quote! {
            Self::#variant_ident => serde_json::json!([#path, operator, value])
        },
    }
}
