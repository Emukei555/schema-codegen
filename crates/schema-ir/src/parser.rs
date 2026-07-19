use crate::{Field, PrimitiveType, Schema, Table};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FbsParser;

pub fn parse_fbs(input: &str) -> Result<Schema, Box<dyn std::error::Error>> {
    // fileルールを起点にパース開始
    let mut file_pairs = FbsParser::parse(Rule::file, input)?;
    let file_inner = file_pairs.next().unwrap().into_inner();

    let mut parsed_tables = Vec::new();

    for record in file_inner {
        // MVP0では table_def が1つだけ来る想定
        if let Rule::table_def = record.as_rule() {
            let mut inner = record.into_inner();

            let table_name = inner.next().unwrap().as_str().to_string();
            let mut fields = Vec::new();

            // 残りの要素は field ルール
            for field_pair in inner {
                if let Rule::field = field_pair.as_rule() {
                    let mut field_inner = field_pair.into_inner();
                    let field_name = field_inner.next().unwrap().as_str().to_string();
                    let type_str = field_inner.next().unwrap().as_str();

                    let field_type = match type_str {
                        "float" => PrimitiveType::Float,
                        "int" => PrimitiveType::Int,
                        "bool" => PrimitiveType::Bool,
                        "string" => PrimitiveType::String,
                        _ => unreachable!("Grammar prevents this"),
                    };

                    fields.push(Field {
                        name: field_name,
                        field_type,
                    });
                }
            }
            parsed_tables.push(Table {
                name: table_name,
                fields,
            });
        }
    }

    Ok(Schema {
        tables: parsed_tables,
    })
}
