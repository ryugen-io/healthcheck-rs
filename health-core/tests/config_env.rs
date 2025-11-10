use healthcheck_core::config::{DEFAULT_DB_PORT, DEFAULT_HTTP_TIMEOUT_MS, DbConfig, HttpConfig};

#[test]
fn http_timeout_invalid_value_falls_back_to_default() {
    let key = "METAMCP_HTTP_TIMEOUT_MS";
    unsafe { std::env::set_var(key, "not-a-number") };

    let http = HttpConfig::from_env();
    assert_eq!(http.timeout().as_millis() as u64, DEFAULT_HTTP_TIMEOUT_MS);

    unsafe { std::env::remove_var(key) };
}

#[test]
fn db_port_invalid_value_falls_back_to_default() {
    let key = "POSTGRES_PORT";
    unsafe { std::env::set_var(key, "99999") };

    let db = DbConfig::from_env();
    assert_eq!(db.port(), DEFAULT_DB_PORT);

    unsafe { std::env::remove_var(key) };
}
