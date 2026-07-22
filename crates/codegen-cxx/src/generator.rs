use schema_ir::{EnumDef, ObjectDef, PrimitiveType, Schema, TypeRef};
use std::fmt::Write;

pub fn generate_cxx(schema: &Schema) -> String {
    let mut out = String::new();

    out.push_str("#pragma once\n\n");
    out.push_str("#include <cstdint>\n");
    out.push_str("#include <string>\n");
    out.push_str("#include <vector>\n\n");

    for enum_def in &schema.enums {
        out.push_str(&generate_enum(enum_def));
        out.push('\n');
    }

    for object_def in &schema.objects {
        out.push_str(&generate_object(object_def));
        out.push('\n');
    }

    out
}

pub fn generate_enum(enum_def: &EnumDef) -> String {
    let mut s = String::new();

    let base_type_str = match enum_def.base_type {
        schema_ir::PrimitiveType::Bool => "bool",
        schema_ir::PrimitiveType::Byte => "int8_t",
        schema_ir::PrimitiveType::Int => "int32_t",
        schema_ir::PrimitiveType::Float => "float",
        schema_ir::PrimitiveType::String => "std::string",
    };

    writeln!(
        &mut s,
        "enum class {} : {} {{",
        enum_def.name, base_type_str
    )
    .unwrap();

    for variant in &enum_def.variants {
        writeln!(&mut s, "    {} = {},", variant.name, variant.value).unwrap();
    }

    writeln!(&mut s, "}};").unwrap();

    s
}

pub fn generate_object(object_def: &ObjectDef) -> String {
    let mut s = String::new();

    writeln!(&mut s, "struct {} {{", object_def.name).unwrap();

    for field in &object_def.fields {
        let cpp_type = to_cpp_type(&field.field_type);
        writeln!(&mut s, "    {} {};", cpp_type, field.name).unwrap();
    }

    writeln!(&mut s, "}};").unwrap();

    s
}

fn to_cpp_type(type_ref: &TypeRef) -> String {
    match type_ref {
        TypeRef::Primitive(PrimitiveType::Bool) => "bool".to_string(),
        TypeRef::Primitive(PrimitiveType::Byte) => "int8_t".to_string(),
        TypeRef::Primitive(PrimitiveType::Int) => "int32_t".to_string(),
        TypeRef::Primitive(PrimitiveType::Float) => "float".to_string(),
        TypeRef::Primitive(PrimitiveType::String) => "std::string".to_string(),

        TypeRef::Obj(name) => name.clone(),

        TypeRef::Vector(inner) => format!("std::vector<{}>", to_cpp_type(inner)),
    }
}

pub fn generate_schema(schema: &Schema) -> String {
    let mut s = String::new();

    s.push_str("#pragma once\n\n");
    s.push_str("#include <cstdint>\n");
    s.push_str("#include <string>\n");
    s.push_str("#include <vector>\n\n");

    for enum_def in &schema.enums {
        s.push_str(&generate_enum(enum_def));
        s.push('\n');
    }

    for obj_def in &schema.objects {
        s.push_str(&generate_object(obj_def));
        s.push('\n');
    }

    s
}
