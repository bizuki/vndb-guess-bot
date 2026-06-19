use darling::FromMeta;
use heck::ToSnakeCase;
use proc_macro2::Ident;
use syn::{
    spanned::Spanned, Data, DeriveInput, Error, Fields, GenericArgument, Meta, PathArguments,
    Result, Type, Variant,
};

use crate::input;

use super::{filter_ops, FilterTarget, Selection, SelectionTarget};

#[derive(Debug, FromMeta)]
struct FilterVariantArgs {
    #[darling(default)]
    field: Option<String>,
    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    nested: bool,
    #[darling(default)]
    flatten: bool,
    #[darling(default)]
    ordered: bool,
    #[darling(default)]
    nullable: bool,
    #[darling(default)]
    value: Option<input::FilterValueType>,
}

pub fn collect(input: &DeriveInput) -> Result<Vec<Selection>> {
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return Err(Error::new(
                input.ident.span(),
                "#[derive(VndbFiltersEnum)] can only be used on user-defined filter enums",
            ));
        }
    };

    variants.iter().map(collect_variant).collect()
}

fn collect_variant(variant: &Variant) -> Result<Selection> {
    let args = filter_variant_args(variant)?;

    if args.nested && args.flatten {
        return Err(Error::new(
            variant.span(),
            "`#[vndb_filter(nested)]` cannot be combined with `flatten`",
        ));
    }

    let payload = filter_variant_payload(variant)?;
    let nested_enum_ident = payload.as_ref().map(|payload| payload.ident.clone());
    let nested_boxed = payload
        .as_ref()
        .map(|payload| payload.boxed)
        .unwrap_or(false);

    if (args.nested || args.flatten) && nested_enum_ident.is_none() {
        return Err(Error::new(
            variant.span(),
            "nested or flattened filter variants must carry one child filter enum",
        ));
    }

    if !(args.nested || args.flatten) && nested_enum_ident.is_some() {
        return Err(Error::new(
            variant.span(),
            "filter variants with payloads must be marked nested or flatten",
        ));
    }

    let filter_value_ty = if args.nested || args.flatten {
        None
    } else {
        Some(
            args.value
                .ok_or_else(|| {
                    Error::new(
                        variant.span(),
                        "leaf filter variants must specify `value = TypePath`",
                    )
                })?
                .ty,
        )
    };

    let path = args
        .field
        .or(args.rename)
        .unwrap_or_else(|| variant.ident.to_string().to_snake_case());

    Ok(Selection {
        variant_ident: variant.ident.clone(),
        path,
        nested_enum_ident,
        nested_boxed,
        flattened: args.flatten,
        target: SelectionTarget::Filter(FilterTarget {
            ops: filter_ops(args.ordered),
            value_ty: filter_value_ty,
            nullable: args.nullable,
        }),
    })
}

fn filter_variant_args(variant: &Variant) -> Result<FilterVariantArgs> {
    let attr = variant
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("vndb_filter"))
        .ok_or_else(|| {
            Error::new(
                variant.span(),
                "filter variants must declare `#[vndb_filter(...)]` metadata",
            )
        })?;

    match &attr.meta {
        Meta::Path(_) => Ok(FilterVariantArgs {
            field: None,
            rename: None,
            nested: false,
            flatten: false,
            ordered: false,
            nullable: false,
            value: None,
        }),
        meta => FilterVariantArgs::from_meta(meta).map_err(Error::from),
    }
}

struct FilterPayload {
    ident: Ident,
    boxed: bool,
}

fn filter_variant_payload(variant: &Variant) -> Result<Option<FilterPayload>> {
    match &variant.fields {
        Fields::Unit => Ok(None),
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let field = fields.unnamed.first().expect("length checked");
            filter_payload(&field.ty).map(Some)
        }
        Fields::Unnamed(_) => Err(Error::new(
            variant.span(),
            "filter variants can carry at most one child filter enum",
        )),
        Fields::Named(_) => Err(Error::new(
            variant.span(),
            "filter variants cannot use named fields",
        )),
    }
}

fn filter_payload(ty: &Type) -> Result<FilterPayload> {
    match ty {
        Type::Path(path) if path.qself.is_none() => {
            let segment = path.path.segments.last().ok_or_else(|| {
                Error::new_spanned(ty, "filter variant payload must be a child filter enum")
            })?;

            match &segment.arguments {
                PathArguments::None => Ok(FilterPayload {
                    ident: segment.ident.clone(),
                    boxed: false,
                }),
                PathArguments::AngleBracketed(arguments) if segment.ident == "Box" => {
                    let mut arguments = arguments.args.iter();
                    let Some(GenericArgument::Type(inner_ty)) = arguments.next() else {
                        return Err(Error::new_spanned(
                            &segment.arguments,
                            "`Box` filter variant payload must wrap one child filter enum",
                        ));
                    };

                    if arguments.next().is_some() {
                        return Err(Error::new_spanned(
                            &segment.arguments,
                            "`Box` filter variant payload must wrap one child filter enum",
                        ));
                    }

                    filter_payload(inner_ty).and_then(|payload| {
                        if payload.boxed {
                            Err(Error::new_spanned(
                                inner_ty,
                                "filter variant payload cannot use nested `Box` storage",
                            ))
                        } else {
                            Ok(FilterPayload {
                                ident: payload.ident,
                                boxed: true,
                            })
                        }
                    })
                }
                _ => Err(Error::new_spanned(
                    &segment.arguments,
                    "filter variant payload cannot have generic arguments except `Box<ChildFilters>`",
                )),
            }
        }
        _ => Err(Error::new_spanned(
            ty,
            "filter variant payload must be a child filter enum",
        )),
    }
}
