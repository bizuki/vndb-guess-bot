use darling::{Error, FromField, FromMeta};
use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute, DeriveInput, Expr, Field, Ident, Type, TypePath};

#[derive(Debug, Clone, Copy)]
pub enum SelectionKind {
    Field,
    Sort,
}

impl SelectionKind {
    pub fn attr_name(self) -> &'static str {
        match self {
            Self::Field => "vndb_field",
            Self::Sort => "vndb_sort",
        }
    }
}

#[derive(Debug)]
pub struct SelectionInput {
    pub ident: Option<Ident>,
    pub ty: Type,
    pub present: bool,
    pub skip: bool,
    pub nested: bool,
    pub flatten: bool,
    pub boxed: bool,
    pub rename: Option<String>,
}

#[derive(Debug)]
pub struct SortExtraInput {
    pub path: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FilterValueType {
    pub ty: TypePath,
}

impl FromMeta for FilterValueType {
    fn from_expr(expr: &Expr) -> darling::Result<Self> {
        match expr {
            Expr::Path(path) if path.attrs.is_empty() => Ok(Self {
                ty: TypePath {
                    qself: path.qself.clone(),
                    path: path.path.clone(),
                },
            }),
            Expr::Group(group) => Self::from_expr(&group.expr),
            Expr::Lit(_) => Err(Error::custom(
                "filter value must be a Rust type path, for example `value = Birthday`",
            )
            .with_span(expr)),
            _ => Err(Error::unexpected_expr_type(expr)),
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(vndb_field))]
struct FieldInput {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    skip: bool,
    #[darling(default)]
    nested: bool,
    #[darling(default)]
    flatten: bool,
    #[darling(default)]
    boxed: bool,
    #[darling(default)]
    rename: Option<String>,
}

#[derive(Debug, FromField)]
#[darling(attributes(vndb_sort))]
struct SortInput {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    skip: bool,
    #[darling(default)]
    nested: bool,
    #[darling(default)]
    flatten: bool,
    #[darling(default)]
    rename: Option<String>,
}

#[derive(Debug, FromMeta)]
struct SortExtraArgs {
    field: String,
}

pub fn parse(field: &Field, kind: SelectionKind) -> darling::Result<SelectionInput> {
    let present = has_attr(&field.attrs, kind.attr_name());

    match kind {
        SelectionKind::Field => FieldInput::from_field(field).map(|input| SelectionInput {
            ident: input.ident,
            ty: input.ty,
            present,
            skip: input.skip,
            nested: input.nested,
            flatten: input.flatten,
            boxed: input.boxed,
            rename: input.rename,
        }),
        SelectionKind::Sort => SortInput::from_field(field).map(|input| SelectionInput {
            ident: input.ident,
            ty: input.ty,
            present,
            skip: input.skip,
            nested: input.nested,
            flatten: input.flatten,
            boxed: false,
            rename: input.rename,
        }),
    }
}

pub fn parse_sort_extras(input: &DeriveInput) -> darling::Result<Vec<SortExtraInput>> {
    input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("vndb_sort"))
        .map(|attr| {
            SortExtraArgs::from_meta(&attr.meta).map(|args| SortExtraInput {
                path: args.field,
                span: attr.path().span(),
            })
        })
        .collect()
}

fn has_attr(attrs: &[Attribute], attr_name: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(attr_name))
}
