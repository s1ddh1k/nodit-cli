use anyhow::{anyhow, Result};
use serde_json::{json, Value};

use crate::cli::OutputFormat;

pub fn render_success(value: &Value, output: OutputFormat, field: Option<&str>) -> Result<()> {
    render_value(&envelope_success(value), output, field)
}

pub fn render_error(
    error: &anyhow::Error,
    output: OutputFormat,
    field: Option<&str>,
) -> Result<()> {
    render_value(&envelope_error(error), output, field)
}

fn render_value(value: &Value, output: OutputFormat, field: Option<&str>) -> Result<()> {
    let selected = match field {
        Some(path) => select_field(value, path)?,
        None => value,
    };

    match output {
        OutputFormat::Pretty => {
            println!("{}", serde_json::to_string_pretty(selected)?);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(selected)?);
        }
        OutputFormat::Raw => match selected {
            Value::String(text) => println!("{text}"),
            Value::Null => println!("null"),
            Value::Bool(value) => println!("{value}"),
            Value::Number(value) => println!("{value}"),
            _ => println!("{}", serde_json::to_string(selected)?),
        },
    }

    Ok(())
}

fn envelope_success(value: &Value) -> Value {
    json!({
        "ok": true,
        "data": value,
    })
}

fn envelope_error(error: &anyhow::Error) -> Value {
    json!({
        "ok": false,
        "error": parse_error_value(error),
    })
}

fn parse_error_value(error: &anyhow::Error) -> Value {
    let text = error.to_string();
    serde_json::from_str::<Value>(&text).unwrap_or_else(|_| json!({ "message": text }))
}

fn select_field<'a>(value: &'a Value, field: &str) -> Result<&'a Value> {
    let field = field.trim();
    if field.is_empty() {
        return Err(anyhow!("field path must not be empty"));
    }

    for candidate in expand_field_paths(field) {
        if let Ok(selected) = select_field_path(value, &candidate) {
            return Ok(selected);
        }
    }

    select_field_path(value, &expand_field_paths(field)[0])
}

fn expand_field_paths(field: &str) -> Vec<String> {
    match field {
        "ok" | "data" | "error" => vec![field.to_string()],
        "status" => vec!["data.status".to_string(), "error.status".to_string()],
        "headers" => vec!["data.headers".to_string()],
        "body" => vec!["data.body".to_string(), "error.body".to_string()],
        "result" => vec!["data.body.result".to_string()],
        path if path.starts_with("body.") => {
            vec![format!("data.{path}"), format!("error.{path}")]
        }
        path if path.starts_with("headers.") => vec![format!("data.{path}")],
        path if path.starts_with("result.") => vec![format!("data.body.{path}")],
        path => vec![path.to_string()],
    }
}

fn select_field_path<'a>(value: &'a Value, field: &str) -> Result<&'a Value> {
    let mut current = value;
    for segment in field.split('.') {
        current = select_segment(current, segment)?;
    }
    Ok(current)
}

fn select_segment<'a>(value: &'a Value, segment: &str) -> Result<&'a Value> {
    if segment.is_empty() {
        return Err(anyhow!("field path contains an empty segment"));
    }

    if let Ok(index) = segment.parse::<usize>() {
        let Value::Array(items) = value else {
            return Err(anyhow!("field segment '{segment}' requires an array"));
        };
        return items
            .get(index)
            .ok_or_else(|| anyhow!("field index out of range: {segment}"));
    }

    let Value::Object(object) = value else {
        return Err(anyhow!("field segment '{segment}' requires an object"));
    };

    object
        .get(segment)
        .ok_or_else(|| anyhow!("field not found: {segment}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_field_reads_nested_object() {
        let value = json!({
            "ok": true,
            "data": {
                "body": {
                    "result": {
                        "number": "0x10"
                    }
                }
            }
        });

        assert_eq!(
            select_field(&value, "data.body.result.number").expect("field"),
            "0x10"
        );
    }

    #[test]
    fn select_field_reads_array_index() {
        let value = json!({
            "data": {
                "items": ["a", "b", "c"]
            }
        });

        assert_eq!(select_field(&value, "data.items.1").expect("field"), "b");
    }

    #[test]
    fn select_field_expands_result_alias() {
        let value = json!({
            "data": {
                "body": {
                    "result": {
                        "value": 7
                    }
                }
            }
        });

        assert_eq!(select_field(&value, "result.value").expect("field"), 7);
    }

    #[test]
    fn select_field_expands_status_alias() {
        let value = json!({
            "data": {
                "status": 200
            }
        });

        assert_eq!(select_field(&value, "status").expect("field"), 200);
    }

    #[test]
    fn select_field_expands_status_alias_for_error_envelope() {
        let value = json!({
            "ok": false,
            "error": {
                "status": 401
            }
        });

        assert_eq!(select_field(&value, "status").expect("field"), 401);
    }

    #[test]
    fn parse_error_value_wraps_plain_text() {
        let error = anyhow!("plain failure");
        assert_eq!(
            parse_error_value(&error),
            json!({ "message": "plain failure" })
        );
    }

    #[test]
    fn parse_error_value_preserves_json_text() {
        let error = anyhow!("{}", r#"{"status":401,"body":{"code":"NO_AUTH"}}"#);
        assert_eq!(
            parse_error_value(&error),
            json!({
                "status": 401,
                "body": {
                    "code": "NO_AUTH"
                }
            })
        );
    }
}
