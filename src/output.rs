use anyhow::Result;
use serde_json::Value;

use crate::cli::OutputFormat;

pub fn render_output(value: &Value, output: OutputFormat) -> Result<()> {
    match output {
        OutputFormat::Pretty => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(value)?);
        }
        OutputFormat::Raw => match value {
            Value::String(text) => println!("{text}"),
            _ => println!("{}", serde_json::to_string(value)?),
        },
    }

    Ok(())
}
