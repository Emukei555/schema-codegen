use crate::{EnumDef, EnumVal, Field, ObjectDef, PrimitiveType, Schema, TypeRef};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FbsParser;

fn parse_type(pair: pest::iterators::Pair<Rule>) -> TypeRef {
    match pair.as_rule() {
        Rule::primitive_type => {
            let prim = match pair.as_str() {
                "float" => PrimitiveType::Float,
                "int" => PrimitiveType::Int,
                "bool" => PrimitiveType::Bool,
                "string" => PrimitiveType::String,
                "byte" => PrimitiveType::Byte,
                _ => unreachable!("Grammar prevents this"),
            };
            TypeRef::Primitive(prim)
        }
        Rule::custom_type => TypeRef::Obj(pair.as_str().to_string()),
        Rule::vector_type => {
            let inner_pair = pair.into_inner().next().unwrap();
            TypeRef::Vector(Box::new(parse_type(inner_pair)))
        }
        _ => unreachable!("Grammar prevents this"),
    }
}

pub fn parse_fbs(input: &str) -> Result<Schema, Box<dyn std::error::Error>> {
    let mut file_pairs = FbsParser::parse(Rule::file, input)?;
    let file_inner = file_pairs.next().unwrap().into_inner();

    let mut parsed_objects = Vec::new();
    let mut parsed_enums = Vec::new();

    for record in file_inner {
        match record.as_rule() {
            Rule::table_def | Rule::struct_def => {
                let is_struct = record.as_rule() == Rule::struct_def;

                let mut inner = record.into_inner();
                let obj_name = inner.next().unwrap().as_str().to_string();
                let mut fields = Vec::new();

                for field_pair in inner {
                    if let Rule::field = field_pair.as_rule() {
                        let mut field_inner = field_pair.into_inner();
                        let field_name = field_inner.next().unwrap().as_str().to_string();

                        let type_ref_pair =
                            field_inner.next().unwrap().into_inner().next().unwrap();

                        let field_type = parse_type(type_ref_pair);

                        fields.push(Field {
                            name: field_name,
                            field_type,
                            attributes: Vec::new(),
                        });
                    }
                }

                parsed_objects.push(ObjectDef {
                    name: obj_name,
                    fields,
                    is_struct, // struct_def だった場合は true になる
                    attributes: Vec::new(),
                });
            }
            Rule::enum_def => {
                let mut inner = record.into_inner();
                let enum_name = inner.next().unwrap().as_str().to_string();
                let type_str = inner.next().unwrap().as_str();

                let base_type = match type_str {
                    "float" => PrimitiveType::Float,
                    "int" => PrimitiveType::Int,
                    "bool" => PrimitiveType::Bool,
                    "string" => PrimitiveType::String,
                    "byte" => PrimitiveType::Byte,
                    _ => unreachable!("Grammar prevents this"),
                };

                let variants_pair = inner.next().unwrap();
                let mut variants = Vec::new();

                let mut next_value: i64 = 0;

                for variant_pair in variants_pair.into_inner() {
                    let mut variant_inner = variant_pair.into_inner();
                    let variant_name = variant_inner.next().unwrap().as_str().to_string();

                    if let Some(int_pair) = variant_inner.next() {
                        next_value = int_pair.as_str().parse::<i64>().unwrap();
                    }

                    variants.push(EnumVal {
                        name: variant_name,
                        value: next_value,
                    });

                    next_value += 1;
                }

                parsed_enums.push(EnumDef {
                    name: enum_name,
                    base_type,
                    variants,
                    attributes: Vec::new(),
                });
            }
            _ => {}
        }
    }

    Ok(Schema {
        objects: parsed_objects,
        enums: parsed_enums,
    })
}
