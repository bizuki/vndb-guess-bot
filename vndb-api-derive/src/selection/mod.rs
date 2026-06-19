pub mod fields;
pub mod filter;
pub mod generic_helpers;

use bitmask_enum::bitmask;
use proc_macro2::Ident;
use syn::TypePath;

pub struct Selection {
    pub variant_ident: Ident,
    pub path: String,
    pub nested_enum_ident: Option<Ident>,
    pub nested_boxed: bool,
    pub flattened: bool,
    pub target: SelectionTarget,
}

pub enum SelectionTarget {
    Field(FieldTarget),
    Filter(FilterTarget),
    Sort(SortTarget),
}

pub struct FieldTarget;

pub struct SortTarget;

pub struct FilterTarget {
    pub ops: FilterOperatorSet,
    pub value_ty: Option<TypePath>,
    pub nullable: bool,
}

impl Selection {
    pub fn filter_target(&self) -> &FilterTarget {
        match &self.target {
            SelectionTarget::Filter(target) => target,
            _ => panic!("selection is not a filter target"),
        }
    }
}

#[bitmask(u8)]
pub enum FilterOperatorSet {
    Eq,
    NotEqual,
    Gt,
    Gte,
    Lt,
    Lte,
    Equality = Self::Eq.or(Self::NotEqual).bits,
    Ordering = Self::Equality
        .or(Self::Gt)
        .or(Self::Gte)
        .or(Self::Lt)
        .or(Self::Lte)
        .bits,
}

pub(crate) fn filter_ops(ordered: bool) -> FilterOperatorSet {
    if ordered {
        FilterOperatorSet::Ordering
    } else {
        FilterOperatorSet::Equality
    }
}
