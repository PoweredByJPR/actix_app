use jsonschema::{paths::JSONPointer, JSONSchema};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct ValidationError {
    pub status: u16,
    pub source: ErrorSource,
    pub detail: String,
}

#[derive(Serialize)]
struct ErrorSource {
    pub pointer: String,
}

fn get_error_source(pointer: JSONPointer) -> ErrorSource {
    let mut props: Vec<String> = pointer
        .into_vec()
        .iter()
        .filter(|prop| prop.as_str() != "properties")
        .map(|prop| prop.clone())
        .collect();

    // remove last element
    props.pop().unwrap();

    ErrorSource {
        pointer: props.join("/"),
    }
}

pub fn validate<'a>(schema: JSONSchema, input: &'a Value) -> Result<(), Value> {
    let result = schema.validate(&input);
    if let Err(errors) = result {
        let errors: Vec<ValidationError> = errors
            .into_iter()
            .map(|error| ValidationError {
                status: 422,
                detail: error.to_string(),
                source: get_error_source(error.schema_path),
            })
            .collect();
        return Err(json!({ "errors": errors }));
    }
    Ok(())
}
