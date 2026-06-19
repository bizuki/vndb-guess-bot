use heck::ToUpperCamelCase;
use proc_macro2::Ident;
use quote::format_ident;

pub fn unraw_ident(ident: &Ident) -> String {
    let ident = ident.to_string();
    ident.strip_prefix("r#").unwrap_or(&ident).to_owned()
}

pub fn variant_ident(path: &str, span: proc_macro2::Span) -> Ident {
    format_ident!("{}", path.to_upper_camel_case(), span = span)
}
