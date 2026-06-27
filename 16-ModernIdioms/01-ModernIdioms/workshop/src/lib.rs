use std::sync::LazyLock;
use std::collections::HashMap;

/// Global configuration loaded lazily on first access.
/// This replaces the need for `lazy_static!` or `once_cell` crates.
pub struct Config {
    pub max_retries: u32,
    pub timeout_ms: u64,
    pub feature_flags: HashMap<String, bool>,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config {
        max_retries: 3,
        timeout_ms: 5000,
        feature_flags: HashMap::from([
            ("enable_cache".to_string(), true),
            ("debug_mode".to_string(), false),
        ]),
    }
});

/// Returns a reference to the global configuration.
pub fn get_config() -> &'static Config {
    &CONFIG
}

/// Calculate sliding window sums over a slice.
/// Returns a Vec of sums, where each sum is the sum of `window_size` consecutive elements.
pub fn sliding_sum(data: &[i32], window_size: usize) -> Vec<i32> {
    data.windows(window_size).map(|w| w.iter().sum()).collect()
}

/// Process a message with optional ID and content using if let chains.
/// Returns a formatted string if both are Some, or an error message.
pub fn process_message(id: Option<i32>, content: Option<&str>) -> String {
    if let Some(id) = id
        && let Some(content) = content
    {
        format!("Message {}: {}", id, content)
    } else if id.is_none() && content.is_none() {
        "Missing both ID and content".to_string()
    } else if id.is_none() {
        "Missing message ID".to_string()
    } else {
        "Missing message content".to_string()
    }
}

/// Returns platform-specific information using cfg_select! macro.
pub fn get_platform_info() -> &'static str {
    cfg_select! {
        target_os = "linux" => "Linux",
        target_os = "macos" => "macOS",
        target_os = "windows" => "Windows",
        _ => "Unknown",
    }
}

/// Parsed response type for demonstrating assert_matches!
#[derive(Debug, PartialEq)]
pub enum ParsedResponse {
    Success { code: u32, message: String },
    Error(String),
}

/// Parse a response string in format "STATUS:CODE" or "ERROR:message".
pub fn parse_response(input: &str) -> ParsedResponse {
    if let Some(code_str) = input.strip_prefix("OK:") {
        if let Ok(code) = code_str.parse::<u32>() {
            ParsedResponse::Success { code, message: "OK".to_string() }
        } else {
            ParsedResponse::Error("Invalid code".to_string())
        }
    } else if let Some(msg) = input.strip_prefix("ERROR:") {
        ParsedResponse::Error(msg.to_string())
    } else {
        ParsedResponse::Error("Invalid format".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_lock_initializes_once() {
        // First access initializes
        let config = get_config();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout_ms, 5000);
        
        // Second access returns same instance
        let config2 = get_config();
        assert_eq!(config.max_retries, config2.max_retries);
    }

    #[test]
    fn test_config_feature_flags() {
        let config = get_config();
        assert_eq!(config.feature_flags.get("enable_cache"), Some(&true));
        assert_eq!(config.feature_flags.get("debug_mode"), Some(&false));
    }

    #[test]
    fn test_sliding_sum_basic() {
        let data = vec![1, 2, 3, 4, 5];
        let result = sliding_sum(&data, 3);
        assert_eq!(result, vec![6, 9, 12]); // [1+2+3, 2+3+4, 3+4+5]
    }

    #[test]
    fn test_sliding_sum_window_equals_length() {
        let data = vec![1, 2, 3];
        let result = sliding_sum(&data, 3);
        assert_eq!(result, vec![6]);
    }

    #[test]
    fn test_sliding_sum_window_larger_than_data() {
        let data = vec![1, 2];
        let result = sliding_sum(&data, 3);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_sliding_sum_empty_data() {
        let data: Vec<i32> = vec![];
        let result = sliding_sum(&data, 3);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_process_message_both_some() {
        let result = process_message(Some(1), Some("hello"));
        assert_eq!(result, "Message 1: hello");
    }

    #[test]
    fn test_process_message_no_id() {
        let result = process_message(None, Some("hello"));
        assert_eq!(result, "Missing message ID");
    }

    #[test]
    fn test_process_message_no_content() {
        let result = process_message(Some(1), None);
        assert_eq!(result, "Missing message content");
    }

    #[test]
    fn test_process_message_both_none() {
        let result = process_message(None, None);
        assert_eq!(result, "Missing both ID and content");
    }

    #[test]
    fn test_get_platform_info() {
        let info = get_platform_info();
        assert!(!info.is_empty());
    }

    #[test]
    fn test_parse_response_success() {
        let result = parse_response("OK:42");
        match result {
            ParsedResponse::Success { code, message } => {
                assert_eq!(code, 42);
                assert_eq!(message, "OK");
            }
            _ => panic!("Expected Success variant"),
        }
    }

    #[test]
    fn test_parse_response_error() {
        let result = parse_response("ERROR:not found");
        match result {
            ParsedResponse::Error(msg) => {
                assert_eq!(msg, "not found");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_parse_response_invalid_format() {
        let result = parse_response("INVALID");
        assert!(matches!(result, ParsedResponse::Error(_)));
    }
}