use healthcheck_core::memory::parse_meminfo_value;

#[test]
fn parse_meminfo_value_parses_number() {
    assert_eq!(parse_meminfo_value("MemTotal:      12345 kB"), Some(12_345));
}

#[test]
fn parse_meminfo_value_handles_invalid_input() {
    assert_eq!(parse_meminfo_value("MemTotal:"), None);
    assert_eq!(parse_meminfo_value(""), None);
}
