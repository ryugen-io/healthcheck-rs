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

// Extra capacity for escape sequences in JSON strings
const JSON_ESCAPE_BUFFER: usize = 16;

/// Escape special characters in JSON strings per RFC 8259
///
/// This function is public for testing purposes
fn escape_json_string(s: &str) -> String {
    // Pre-allocate with some extra capacity for escape sequences
    let mut result = String::with_capacity(s.len() + JSON_ESCAPE_BUFFER);

    for ch in s.chars() {
        match ch {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '/' => result.push_str("\\/"), // Solidus (optional but recommended for XSS prevention)
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\u{0008}' => result.push_str("\\b"), // backspace
            '\u{000C}' => result.push_str("\\f"), // form feed
            // Escape all other control characters (U+0000 to U+001F)
            c if c < '\u{0020}' => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => result.push(ch),
        }
    }

    result
}

#[cfg(test)]
mod tests;
