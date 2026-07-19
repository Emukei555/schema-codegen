use schema_ir::{PrimitiveType, Schema, TypeRef};

pub fn generate_rust_code(schema: &Schema) -> String {
    let mut code = String::new();

    for enum_def in &schema.enums {
        let repr_type = match enum_def.base_type {
            PrimitiveType::Byte => "i8",
            PrimitiveType::Int => "i32",
            _ => "i32",
        };

        code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        code.push_str(&format!("#[repr({})]\n", repr_type));
        code.push_str(&format!("pub enum {} {{\n", enum_def.name));

        for variant in &enum_def.variants {
            code.push_str(&format!("    {} = {},\n", variant.name, variant.value));
        }

        code.push_str("}\n\n");
    }

    for object in &schema.objects {
        code.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        code.push_str(&format!("pub struct {} {{\n", object.name));

        for field in &object.fields {
            let rust_type = resolve_rust_type(&field.field_type);
            code.push_str(&format!("    pub {}: {},\n", field.name, rust_type));
        }

        code.push_str("}\n\n");
    }

    code
}

fn resolve_rust_type(type_ref: &TypeRef) -> String {
    match type_ref {
        TypeRef::Primitive(PrimitiveType::Float) => "f32".to_string(),
        TypeRef::Primitive(PrimitiveType::Int) => "i32".to_string(),
        TypeRef::Primitive(PrimitiveType::Bool) => "bool".to_string(),
        TypeRef::Primitive(PrimitiveType::String) => "String".to_string(),
        TypeRef::Primitive(PrimitiveType::Byte) => "i8".to_string(),
        TypeRef::Obj(name) => name.clone(),
        TypeRef::Vector(inner) => {
            format!("Vec<{}>", resolve_rust_type(inner))
        }
    }
}
