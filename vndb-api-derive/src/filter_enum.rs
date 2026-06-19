use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{
    expand, path,
    selection::{self, Selection},
};

pub fn derive_filter_enum(input: DeriveInput) -> TokenStream {
    match expand_filter_enum(&input) {
        Ok(expanded) => expanded,
        Err(error) => error.to_compile_error(),
    }
}

fn expand_filter_enum(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_ident = &input.ident;
    let selections = selection::filter::collect(input)?;
    let display_arms = selections.iter().map(display_arm);
    let flattened_from_impls = expand::flattened_from_impls(enum_ident, &selections);
    let path_helpers = path::helpers(
        enum_ident,
        &input.vis,
        &selections,
        path::PathConfig {
            derive_name: "VndbFiltersEnum",
        },
    );

    Ok(quote! {
        impl serde::Serialize for #enum_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #( #display_arms, )*
                }
            }
        }

        #( #flattened_from_impls )*

        #path_helpers
    })
}

fn display_arm(selection: &Selection) -> TokenStream {
    let variant_ident = &selection.variant_ident;
    let path = &selection.path;

    if selection.flattened {
        return quote! {
            Self::#variant_ident(item) => std::fmt::Display::fmt(item, f)
        };
    }

    match &selection.nested_enum_ident {
        Some(_) => quote! {
            Self::#variant_ident(item) => {
                f.write_str(#path)?;
                f.write_str("{")?;
                std::fmt::Display::fmt(item, f)?;
                f.write_str("}")
            }
        },
        None => quote! {
            Self::#variant_ident => f.write_str(#path)
        },
    }
}
