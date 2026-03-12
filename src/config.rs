use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::cli::Args;

const DEFAULT_API_BASE_URL: &str = "https://web3.nodit.io";
const DEFAULT_STREAM_URL: &str = "wss://web3.nodit.io";
const DEFAULT_APTOS_API_BASE_URL: &str = "https://aptos-mainnet.nodit.io/v1";

#[derive(Clone, Debug)]
pub struct Config {
    pub api_key: Option<String>,
    pub api_base_url: String,
    pub rpc_url: Option<String>,
    pub stream_url: Option<String>,
    pub stream_base_url: String,
    pub aptos_api_base_url: String,
}

#[derive(Debug, Default, Deserialize)]
struct FileConfig {
    api_key: Option<String>,
    api_base_url: Option<String>,
    rpc_url: Option<String>,
    stream_url: Option<String>,
    aptos_api_base_url: Option<String>,
}

impl Config {
    pub fn from_env(args: &Args) -> Result<Self> {
        let file_config = load_config_file()?;
        let dotenv = load_dotenv()?;

        let api_key = args
            .api_key
            .clone()
            .or_else(|| env::var("NODIT_API_KEY").ok())
            .or_else(|| dotenv.get("NODIT_API_KEY").cloned())
            .or(file_config.api_key);

        let api_base_url = args
            .api_base_url
            .clone()
            .or_else(|| env::var("NODIT_API_BASE_URL").ok())
            .or_else(|| dotenv.get("NODIT_API_BASE_URL").cloned())
            .or(file_config.api_base_url)
            .unwrap_or_else(|| DEFAULT_API_BASE_URL.to_string());

        let rpc_url = args
            .rpc_url
            .clone()
            .or_else(|| env::var("NODIT_RPC_URL").ok())
            .or_else(|| dotenv.get("NODIT_RPC_URL").cloned())
            .or(file_config.rpc_url);

        let stream_url = args
            .stream_url
            .clone()
            .or_else(|| env::var("NODIT_STREAM_URL").ok())
            .or_else(|| dotenv.get("NODIT_STREAM_URL").cloned())
            .or(file_config.stream_url);

        let aptos_api_base_url = args
            .aptos_api_base_url
            .clone()
            .or_else(|| env::var("NODIT_APTOS_API_BASE_URL").ok())
            .or_else(|| dotenv.get("NODIT_APTOS_API_BASE_URL").cloned())
            .or(file_config.aptos_api_base_url)
            .unwrap_or_else(|| DEFAULT_APTOS_API_BASE_URL.to_string());

        Ok(Self {
            api_key,
            api_base_url,
            rpc_url,
            stream_base_url: stream_url
                .clone()
                .unwrap_or_else(|| DEFAULT_STREAM_URL.to_string()),
            stream_url,
            aptos_api_base_url,
        })
    }
}

fn load_config_file() -> Result<FileConfig> {
    let Some(path) = config_file_path() else {
        return Ok(FileConfig::default());
    };
    if !path.exists() {
        return Ok(FileConfig::default());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    toml::from_str(&content)
        .with_context(|| format!("failed to parse config file: {}", path.display()))
}

fn load_dotenv() -> Result<HashMap<String, String>> {
    let path = Path::new(".env");
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    parse_dotenv(&content)
}

fn config_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("nodit-cli").join("config.toml"))
}

fn parse_dotenv(content: &str) -> Result<HashMap<String, String>> {
    let mut values = HashMap::new();

    for (index, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let Some((key, value)) = trimmed.split_once('=') else {
            anyhow::bail!("invalid .env line {}: expected KEY=VALUE", index + 1);
        };

        let key = key.trim().to_string();
        let value = strip_quotes(value.trim()).to_string();
        values.insert(key, value);
    }

    Ok(values)
}

fn strip_quotes(value: &str) -> &str {
    if value.len() >= 2 {
        let bytes = value.as_bytes();
        if (bytes[0] == b'"' && bytes[value.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[value.len() - 1] == b'\'')
        {
            return &value[1..value.len() - 1];
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dotenv_ignores_comments_and_quotes() {
        let parsed = parse_dotenv(
            r#"
# comment
NODIT_API_KEY="abc"
NODIT_API_BASE_URL=https://web3.nodit.io
"#,
        )
        .expect("dotenv should parse");

        assert_eq!(parsed.get("NODIT_API_KEY"), Some(&"abc".to_string()));
        assert_eq!(
            parsed.get("NODIT_API_BASE_URL"),
            Some(&"https://web3.nodit.io".to_string())
        );
    }
}
