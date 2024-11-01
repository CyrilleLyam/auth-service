use serde_json::json;
use serde_json::Value;
use validator::ValidationErrors;

pub fn success_response(message: &str, data: Value) -> Value {
    json!({
        "status":true,
        "message": message,
        "data": data
    })
}

pub fn error_response(message: &str, details: Value) -> Value {
    json!({
        "status":false,
        "message": message,
        "data": details
    })
}

pub fn validation_errors_to_json(errors: ValidationErrors) -> Value {
    let mut error_map = serde_json::Map::new();
    for (field, errors) in errors.field_errors() {
        let messages: Vec<String> = errors
            .iter()
            .map(|e| e.message.clone().unwrap_or_default().to_string())
            .collect();
        error_map.insert(field.to_string(), json!(messages));
    }
    json!(error_map)
}
