use baml_types::{ir_type::TypeNonStreaming, BamlMediaType, TypeValue};
use convert_case::{Case, Casing};

/// Convert IR type to C# type string and nullability
pub fn type_to_csharp_string(ir_type: &TypeNonStreaming) -> (String, bool) {
    use TypeNonStreaming as T;

    let type_str = match ir_type {
        T::Primitive(type_value, _) => match type_value {
            TypeValue::String => "string".to_string(),
            TypeValue::Int => "int".to_string(),
            TypeValue::Float => "double".to_string(),
            TypeValue::Bool => "bool".to_string(),
            TypeValue::Null => "object".to_string(),
            TypeValue::Media(media_type) => match media_type {
                BamlMediaType::Image => "BamlImage".to_string(),
                BamlMediaType::Audio => "BamlAudio".to_string(),
                BamlMediaType::Video => "BamlVideo".to_string(),
                BamlMediaType::Pdf => "BamlPdf".to_string(),
            },
        },
        T::Enum { name, .. } => to_pascal_case(name),
        T::Class { name, .. } => to_pascal_case(name),
        T::List(inner, _) => {
            let (inner_type, _) = type_to_csharp_string(inner);
            format!("List<{}>", inner_type)
        }
        T::Map(key, value, _) => {
            let (key_type, _) = type_to_csharp_string(key);
            let (value_type, _) = type_to_csharp_string(value);
            format!("Dictionary<{}, {}>", key_type, value_type)
        }
        T::Union(union_type, _) => {
            match union_type.view() {
                baml_types::ir_type::UnionTypeViewGeneric::Optional(inner) => {
                    // Handle optional types - nullability is returned separately
                    let (inner_type, _) = type_to_csharp_string(inner);
                    return (inner_type, true);
                }
                baml_types::ir_type::UnionTypeViewGeneric::Null => {
                    return ("object".to_string(), true);
                }
                baml_types::ir_type::UnionTypeViewGeneric::OneOf(_)
                | baml_types::ir_type::UnionTypeViewGeneric::OneOfOptional(_) => {
                    // C# doesn't have union types, use object
                    "object".to_string()
                }
            }
        }
        T::RecursiveTypeAlias { name, .. } => to_pascal_case(name),
        T::Literal(_, _) => "object".to_string(),
        T::Tuple(..) => "object".to_string(), // C# doesn't have tuples in the same way
        T::Arrow(..) => "object".to_string(), // Function types become object
        T::Top(_) => {
            // Top should have been resolved by compiler before code generation
            panic!("TypeGeneric::Top should have been resolved by the compiler before code generation")
        }
    };

    // Check if the type itself is optional
    let is_nullable = ir_type.is_optional();

    (type_str, is_nullable)
}

/// Convert to PascalCase for C# type names
pub fn to_pascal_case(name: &str) -> String {
    name.to_case(Case::Pascal)
}

/// Convert to camelCase for C# parameter names
pub fn to_camel_case(name: &str) -> String {
    name.to_case(Case::Camel)
}
