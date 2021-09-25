use jsonschema::JSONSchema;
use serde_json::{json, Value};

use crate::schemas::validator;

fn schema() -> JSONSchema {
    let schema = json!({
        "type": "object",
        "required": vec!["data"],
        "properties": {
            "data": {
                "type": "object",
                "required": vec!["attributes"],
                "properties": {
                    "attributes": {
                        "type": "object",
                        "required": vec!["first_name", "last_name"],
                        "properties": {
                            "first_name": {
                                "type": "string"
                            },
                            "last_name": {
                                "type": "string"
                            }
                        }
                    }
                }
            }
        }
    });
    JSONSchema::compile(&schema).unwrap()
}

pub fn validate<'a>(input: &'a Value) -> Result<(), Value> {
    validator::validate(schema(), &input)
}
