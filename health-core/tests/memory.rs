use healthcheck_core::memory::parse_cgroup_bytes;

#[test]
fn parse_cgroup_bytes_supports_numbers_and_max() {
    assert_eq!(parse_cgroup_bytes("1048576\n"), Some(1_048_576));
    assert_eq!(parse_cgroup_bytes(" max "), None);
    assert_eq!(parse_cgroup_bytes(""), None);
}
