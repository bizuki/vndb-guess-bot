use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod expand;
mod filter_enum;
mod input;
mod path;
mod selection;
mod ty;

#[proc_macro_derive(VndbFieldsEnum, attributes(vndb_field))]
#[proc_macro_error]
pub fn derive_vndb_fields_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive_enum(input, expand::EnumKind::Fields).into()
}

#[proc_macro_derive(VndbFiltersEnum, attributes(vndb_filter))]
#[proc_macro_error]
pub fn derive_vndb_filters_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    filter_enum::derive_filter_enum(input).into()
}

#[proc_macro_derive(VndbSortEnum, attributes(vndb_filter, vndb_sort))]
#[proc_macro_error]
pub fn derive_vndb_sort_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive_enum(input, expand::EnumKind::Sort).into()
}
