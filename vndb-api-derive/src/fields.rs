use darling::FromField;
use heck::ToUpperCamelCase;
use proc_macro_error::abort;
use quote::format_ident;
use syn::{GenericArgument, Ident, PathArguments, Type};

#[derive(Debug, FromField)]
#[darling(attributes(vndb_field))]
pub struct FieldInput {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    #[darling(default)]
    pub skip: bool,
    #[darling(default)]
    pub is_primitive: bool
}

pub struct FieldInfo {
    pub name: Ident,
    pub path: String,
    // whether the field contains other vndbfield
    pub referenced_type: Option<Ident>,
}

pub fn get_field_info(field: &FieldInput) -> Option<FieldInfo> {
    if field.skip {
        return None
    }
    
    let field_type = get_inner_type(&field.ty);
    let field_name = field.ident.as_ref().unwrap().to_string();

    let is_primitive = is_primitive_ident(&field_type) || field.is_primitive;

    Some(FieldInfo {
        name: format_ident!("{}", field_name.to_upper_camel_case()),
        path: field_name,
        referenced_type: if !is_primitive { Some(format_ident!("{field_type}Fields")) } else { None }
    })
}

// this will unwrap all containers to reveal inner type
fn get_inner_type(ty: &Type) -> String {
    match ty {
        Type::Path(p) => {
            if p.path.segments.len() == 1 {
                let seg = &p.path.segments[0];
                let ident = seg.ident.to_string();

                // Simple wrappers: Option<T>, Vec<T>, Box<T> where T is standard
                if matches!(ident.as_str(), "Option"|"Vec"|"Box") {
                    if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                        if let Some(GenericArgument::Type(inner)) = ab.args.first() {
                            return get_inner_type(inner);
                        }
                    }
                }
                // it means that type is inner and not wrapped in other containers
                return ident
            } else {
                p.path.segments[0].ident.to_string()
            }
        }
        // Arrays like [u8; 32]
        Type::Array(arr) => get_inner_type(&arr.elem),
        _ => abort!(
            ty,
            "#[derive(VndbFieldsEnum)] unsupported field type"
        ),
    }
}

fn is_primitive_ident(ident: &str) -> bool {
    matches!(ident,
        "bool"|"char"|
        "u8"|"u16"|"u32"|"u64"|"u128"|"usize"|
        "i8"|"i16"|"i32"|"i64"|"i128"|"isize"|
        "f32"|"f64"|
        "String"
    )
}
