#[derive(Debug)]
pub struct CheckResult {
    pub check_type: String,
    pub ok: bool,
    pub latency_ms: u64,
    pub error: Option<String>,
}

pub fn print_results(results: &[CheckResult], overall: bool) {
    println!("{{");
    println!("  \"overall\": {},", if overall { "true" } else { "false" });
    println!("  \"checks\": [");

    for (i, result) in results.iter().enumerate() {
        let comma = if i < results.len() - 1 { "," } else { "" };
        println!("    {{");
        println!("      \"type\": \"{}\",", result.check_type);
        println!(
            "      \"ok\": {},",
            if result.ok { "true" } else { "false" }
        );
        println!("      \"latency_ms\": {},", result.latency_ms);

        if let Some(err) = &result.error {
            let escaped = escape_json_string(err);
            println!("      \"error\": \"{}\"", escaped);
        } else {
            println!("      \"error\": null");
        }

        println!("    }}{}", comma);
    }

    println!("  ]");
    println!("}}");
}

pub fn print_error_json(message: &str) {
    let escaped = escape_json_string(message);
    println!("{{");
    println!("  \"overall\": false,");
    println!("  \"error\": \"{}\"", escaped);
    println!("}}");
}

/// Escape special characters in JSON strings more efficiently
fn escape_json_string(s: &str) -> String {
    // Pre-allocate with some extra capacity for escape sequences
    let mut result = String::with_capacity(s.len() + 16);

    for ch in s.chars() {
        match ch {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(ch),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_json_string_no_escapes() {
        assert_eq!(escape_json_string("hello world"), "hello world");
        assert_eq!(escape_json_string(""), "");
        assert_eq!(escape_json_string("123"), "123");
    }

    #[test]
    fn test_escape_json_string_backslash() {
        assert_eq!(escape_json_string("path\\to\\file"), "path\\\\to\\\\file");
        assert_eq!(escape_json_string("\\"), "\\\\");
    }

    #[test]
    fn test_escape_json_string_quotes() {
        assert_eq!(escape_json_string("say \"hello\""), "say \\\"hello\\\"");
        assert_eq!(escape_json_string("\""), "\\\"");
    }

    #[test]
    fn test_escape_json_string_newlines() {
        assert_eq!(escape_json_string("line1\nline2"), "line1\\nline2");
        assert_eq!(escape_json_string("\n"), "\\n");
    }

    #[test]
    fn test_escape_json_string_carriage_return() {
        assert_eq!(escape_json_string("line1\rline2"), "line1\\rline2");
        assert_eq!(escape_json_string("\r\n"), "\\r\\n");
    }

    #[test]
    fn test_escape_json_string_tabs() {
        assert_eq!(escape_json_string("col1\tcol2"), "col1\\tcol2");
        assert_eq!(escape_json_string("\t"), "\\t");
    }

    #[test]
    fn test_escape_json_string_mixed() {
        let input = "Error: \"connection\\failed\"\nDetails:\tN/A\r\n";
        let expected = "Error: \\\"connection\\\\failed\\\"\\nDetails:\\tN/A\\r\\n";
        assert_eq!(escape_json_string(input), expected);
    }

    #[test]
    fn test_escape_json_string_unicode() {
        assert_eq!(escape_json_string("hello 世界"), "hello 世界");
        assert_eq!(escape_json_string("emoji 🚀"), "emoji 🚀");
    }
}
