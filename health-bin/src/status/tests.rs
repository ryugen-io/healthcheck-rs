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
    let expected = "Error: \\\"connection\\\\failed\\\"\\nDetails:\\tN\\/A\\r\\n";
    assert_eq!(escape_json_string(input), expected);
}

#[test]
fn test_escape_json_string_unicode() {
    assert_eq!(escape_json_string("hello 世界"), "hello 世界");
    assert_eq!(escape_json_string("emoji 🚀"), "emoji 🚀");
}

#[test]
fn test_escape_json_string_control_chars() {
    // Backspace and form feed
    assert_eq!(escape_json_string("\u{0008}"), "\\b");
    assert_eq!(escape_json_string("\u{000C}"), "\\f");

    // Other control characters
    assert_eq!(escape_json_string("\u{0000}"), "\\u0000"); // null
    assert_eq!(escape_json_string("\u{0001}"), "\\u0001"); // SOH
    assert_eq!(escape_json_string("\u{001F}"), "\\u001f"); // unit separator

    // Mixed with normal text
    assert_eq!(escape_json_string("test\u{0000}data"), "test\\u0000data");
}

#[test]
fn test_escape_json_string_solidus() {
    // Forward slash (solidus) - optional but recommended for XSS prevention
    assert_eq!(escape_json_string("/"), "\\/");
    assert_eq!(
        escape_json_string("http://example.com/path"),
        "http:\\/\\/example.com\\/path"
    );
    assert_eq!(escape_json_string("</script>"), "<\\/script>");
}
