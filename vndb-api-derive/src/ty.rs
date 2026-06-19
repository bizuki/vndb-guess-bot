use proc_macro2::Ident;
use quote::format_ident;
use syn::{spanned::Spanned, Error, GenericArgument, PathArguments, Result, Type};

pub fn nested_enum_ident(ty: &Type, enum_suffix: &str) -> Result<Ident> {
    let inner_ident = inner_type_ident(ty)?;
    Ok(format_ident!(
        "{}{}",
        inner_ident,
        enum_suffix,
        span = ty.span()
    ))
}

fn inner_type_ident(ty: &Type) -> Result<String> {
    match ty {
        Type::Array(array) => inner_type_ident(&array.elem),
        Type::Group(group) => inner_type_ident(&group.elem),
        Type::Paren(paren) => inner_type_ident(&paren.elem),
        Type::Path(path) => {
            let segment = path.path.segments.last().ok_or_else(|| {
                Error::new(path.span(), "expected a type path for nested VNDB fields")
            })?;
            let ident = unraw(&segment.ident);

            if matches!(ident.as_str(), "Option" | "Vec" | "Box") {
                if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner)) = arguments.args.first() {
                        return inner_type_ident(inner);
                    }
                }
            }

            Ok(ident)
        }
        _ => Err(Error::new(
            ty.span(),
            "nested or flattened VNDB fields must use a path type, Option<T>, Vec<T>, Box<T>, or array",
        )),
    }
}

fn unraw(ident: &Ident) -> String {
    let ident = ident.to_string();
    ident.strip_prefix("r#").unwrap_or(&ident).to_owned()
}
