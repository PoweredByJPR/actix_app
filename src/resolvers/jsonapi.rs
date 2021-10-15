use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

#[derive(Serialize, Deserialize)]
pub struct Source {
    pointer: String,
}
#[derive(Serialize, Deserialize)]
pub struct ApiError {
    source: Source,
    detail: String,
    code: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
struct Data<T: Serialize + Validate> {
    #[validate]
    #[validate(required)]
    attributes: Option<T>,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct Model<T: Serialize + Validate> {
    #[validate]
    #[validate(required)]
    data: Option<Data<T>>,
}

impl<T: Serialize + Validate> Model<T> {
    pub fn attributes(self) -> T {
        self.data.unwrap().attributes.unwrap()
    }
}

pub fn map_errors(errors: ValidationErrors, parent: Option<String>) -> Vec<ApiError> {
    let mut error_list: Vec<ApiError> = vec![];

    for (field, kind) in errors.into_errors() {
        match kind {
            ValidationErrorsKind::Field(errors) => {
                let pointer = match &parent {
                    Some(parent) => format!("{}.{}", parent, field),
                    None => field.to_owned(),
                };
                let source = Source { pointer };
                let error = errors.first().unwrap();
                let code = error.code.to_string();
                let detail = error
                    .message
                    .clone()
                    .unwrap_or(format!("{}", &code).into())
                    .to_string();
                error_list.push(ApiError {
                    source,
                    code,
                    detail,
                })
            }
            ValidationErrorsKind::Struct(errors) => {
                let parent = match &parent {
                    Some(parent) => format!("{}.{}", parent, &field),
                    None => field.to_owned(),
                };
                map_errors(*errors, Some(parent))
                    .into_iter()
                    .for_each(|error| error_list.push(error))
            }
            ValidationErrorsKind::List(errors) => errors.into_iter().for_each(|(index, error)| {
                let parent = match &parent {
                    Some(parent) => format!("{}.{}[{}]", parent, &field, index),
                    None => format!("{}[{}].", &field, index),
                };
                map_errors(*error, Some(parent))
                    .into_iter()
                    .for_each(|error| error_list.push(error))
            }),
        }
    }
    error_list
}
