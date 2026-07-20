use schema_ir::{EnumDef, ObjectDef, PrimitiveType, Schema};

pub fn generate_cxx(schema: &Schema) -> String {
    let mut out = String::new();

    out.push_str("#pragma once\n\n");
    out.push_str("#include <cstdint>\n");
    out.push_str("#include <string>\n");
    out.push_str("#include <vector>\n\n");

    for enum_def in &schema.enums {
        // 変数 enum_def の参照を渡す
        out.push_str(&generate_enum(enum_def));
        out.push_str("\n");
    }

    for object_def in &schema.objects {
        out.push_str(&generate_object(object_def));
        out.push_str("\n");
    }

    out
}

fn generate_enum(enum_def: &EnumDef) -> String {
    let mut s = String::new();

    let underlying_type = match enum_def.base_type {
        PrimitiveType::Byte => "int8_t",
        PrimitiveType::Int => "int32_t",
        PrimitiveType::Bool => "bool",
        PrimitiveType::Float => "float",
        PrimitiveType::String => "int32_t",
    };

    s.push_str(&format!(
        "enum class {} : {} {{\n",
        enum_def.name, underlying_type
    ));

    for variant in &enum_def.variants {
        s.push_str(&format!("    {} = {},\n", variant.name, variant.value));
    }

    s.push_str("};\n");

    s
}

// （参考）generate_object もダミーを用意しておくとコンパイルが通ります
fn generate_object(object_def: &ObjectDef) -> String {
    let mut s = String::new();
    s
}
