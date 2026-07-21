use codegen_cxx::generator::{generate_enum, generate_object, generate_schema};
use schema_ir::{EnumDef, EnumVal, Field, ObjectDef, PrimitiveType, Schema, TypeRef};

#[test]
fn test_generate_object() {
    let obj_def = ObjectDef {
        name: "Monster".to_string(),
        is_struct: true,
        attributes: vec![],
        fields: vec![
            Field {
                name: "hp".to_string(),
                field_type: TypeRef::Primitive(PrimitiveType::Int),
                attributes: vec![],
            },
            Field {
                name: "name".to_string(),
                field_type: TypeRef::Primitive(PrimitiveType::String),
                attributes: vec![],
            },
            Field {
                name: "inventory".to_string(),
                field_type: TypeRef::Vector(Box::new(TypeRef::Primitive(PrimitiveType::Byte))),
                attributes: vec![],
            },
        ],
    };

    let result_cpp = generate_object(&obj_def);

    let expected_cpp = "\
struct Monster {
    int32_t hp;
    std::string name;
    std::vector<int8_t> inventory;
};
";

    assert_eq!(result_cpp, expected_cpp);
}

#[test]
fn test_generate_enum() {
    let enum_def = EnumDef {
        name: "Color".to_string(),
        base_type: PrimitiveType::Byte,
        variants: vec![
            EnumVal {
                name: "Red".to_string(),
                value: 0,
            },
            EnumVal {
                name: "Green".to_string(),
                value: 1,
            },
            EnumVal {
                name: "Blue".to_string(),
                value: 2,
            },
        ],
        attributes: vec![],
    };

    let result_cpp = generate_enum(&enum_def);

    let expected_cpp = "\
enum class Color : int8_t {
    Red = 0,
    Green = 1,
    Blue = 2,
};
";

    assert_eq!(result_cpp, expected_cpp);
}

#[test]
fn test_generate_schema() {
    let schema = Schema {
        enums: vec![EnumDef {
            name: "Status".to_string(),
            base_type: PrimitiveType::Byte,
            variants: vec![
                EnumVal {
                    name: "Ok".to_string(),
                    value: 0,
                },
                EnumVal {
                    name: "Error".to_string(),
                    value: 1,
                },
            ],
            attributes: vec![],
        }],
        objects: vec![ObjectDef {
            name: "Response".to_string(),
            is_struct: true,
            attributes: vec![],
            fields: vec![Field {
                name: "message".to_string(),
                field_type: TypeRef::Primitive(PrimitiveType::String),
                attributes: vec![],
            }],
        }],
    };

    let result_cpp = generate_schema(&schema);

    let expected_cpp = "\
#pragma once

#include <cstdint>
#include <string>
#include <vector>

enum class Status : int8_t {
    Ok = 0,
    Error = 1,
};

struct Response {
    std::string message;
};

";

    assert_eq!(result_cpp, expected_cpp);
}
