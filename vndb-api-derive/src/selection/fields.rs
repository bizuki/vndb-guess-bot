use syn::{
    spanned::Spanned, Data, DeriveInput, Error, Field, Fields, GenericArgument, PathArguments,
    Result, Type,
};

use crate::{expand::EnumConfig, input, ty};

use super::{
    generic_helpers::{unraw_ident, variant_ident},
    FieldTarget, Selection, SelectionTarget, SortTarget,
};

pub fn collect(input: &DeriveInput, config: EnumConfig) -> Result<Vec<Selection>> {
    let named_fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(Error::new(
                    input.ident.span(),
                    format!(
                        "#[derive({})] can only be used on structs with named fields",
                        config.derive_name
                    ),
                ));
            }
        },
        _ => {
            return Err(Error::new(
                input.ident.span(),
                format!(
                    "#[derive({})] can only be used on structs",
                    config.derive_name
                ),
            ));
        }
    };

    let mut selections = named_fields
        .iter()
        .filter_map(|field| collect_field(field, config).transpose())
        .collect::<Result<Vec<_>>>()?;

    if config.include_sort_extras {
        selections.extend(
            input::parse_sort_extras(input)
                .map_err(syn::Error::from)?
                .into_iter()
                .map(|extra| {
                    let variant_ident = variant_ident(&extra.path, extra.span);

                    Selection {
                        variant_ident,
                        path: extra.path,
                        nested_enum_ident: None,
                        nested_boxed: false,
                        flattened: false,
                        target: SelectionTarget::Sort(SortTarget),
                    }
                }),
        );
    }

    Ok(selections)
}

fn collect_field(field: &Field, config: EnumConfig) -> Result<Option<Selection>> {
    let parsed = input::parse(field, config.selection_kind).map_err(syn::Error::from)?;
    let ident = parsed.ident.as_ref().ok_or_else(|| {
        Error::new(
            field.span(),
            format!(
                "#[derive({})] can only process named fields",
                config.derive_name
            ),
        )
    })?;

    if parsed.skip && (parsed.nested || parsed.flatten) {
        return Err(Error::new(
            field.span(),
            format!(
                "`#[{}(skip)]` cannot be combined with `nested` or `flatten`",
                config.selection_kind.attr_name()
            ),
        ));
    }

    if parsed.nested && parsed.flatten {
        return Err(Error::new(
            field.span(),
            format!(
                "`#[{}(nested)]` cannot be combined with `flatten`",
                config.selection_kind.attr_name()
            ),
        ));
    }

    if parsed.boxed && !(parsed.nested || parsed.flatten) {
        return Err(Error::new(
            field.span(),
            "`#[vndb_field(boxed)]` requires `nested` or `flatten`",
        ));
    }

    if parsed.skip || (!config.include_by_default && !parsed.present) {
        return Ok(None);
    }

    if parsed.flatten && !config.nested_value.emits_nested_payload() {
        return Ok(None);
    }

    let field_name = unraw_ident(ident);
    let variant_ident = variant_ident(&field_name, ident.span());
    let flattened = parsed.flatten && config.nested_value.emits_nested_payload();
    let nested_enum_ident =
        if (parsed.nested || parsed.flatten) && config.nested_value.emits_nested_payload() {
            Some(ty::nested_enum_ident(
                &parsed.ty,
                config.nested_enum_suffix,
            )?)
        } else {
            None
        };
    let nested_boxed = (parsed.nested || parsed.flatten)
        && config.nested_value.emits_nested_payload()
        && matches!(config.selection_kind, input::SelectionKind::Field)
        && (parsed.boxed || type_contains_box(&parsed.ty));
    let target = match config.selection_kind {
        input::SelectionKind::Field => SelectionTarget::Field(FieldTarget),
        input::SelectionKind::Sort => SelectionTarget::Sort(SortTarget),
    };

    Ok(Some(Selection {
        variant_ident,
        path: parsed.rename.unwrap_or(field_name),
        nested_enum_ident,
        nested_boxed,
        flattened,
        target,
    }))
}

fn type_contains_box(ty: &Type) -> bool {
    match ty {
        Type::Array(array) => type_contains_box(&array.elem),
        Type::Group(group) => type_contains_box(&group.elem),
        Type::Paren(paren) => type_contains_box(&paren.elem),
        Type::Path(path) => {
            let Some(segment) = path.path.segments.last() else {
                return false;
            };
            let ident = segment.ident.to_string();
            let ident = ident.strip_prefix("r#").unwrap_or(&ident);

            if ident == "Box" {
                return true;
            }

            if matches!(ident, "Option" | "Vec") {
                if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner)) = arguments.args.first() {
                        return type_contains_box(inner);
                    }
                }
            }

            false
        }
        _ => false,
    }
}
