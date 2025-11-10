use healthcheck_bin::status::HealthStatus;

#[test]
fn status_json_includes_all_fields() {
    let status = HealthStatus {
        http: true,
        http_latency_ms: 10,
        http_error: Some("oops".into()),
        database: false,
        db_latency_ms: 20,
        db_error: None,
        memory_used_mb: 128,
        memory_total_mb: 256,
        memory_percent: 50.5,
        overall: false,
    };

    let json = status.to_pretty_json();

    assert!(json.contains("\"http\": true"));
    assert!(json.contains("\"http_error\": \"oops\""));
    assert!(json.contains("\"db_error\": null"));
    assert!(json.contains("\"memory_percent\": 50.5"));
}

#[test]
fn status_json_escapes_strings() {
    let status = HealthStatus {
        http: true,
        http_latency_ms: 0,
        http_error: Some("line\n\"quote\"".into()),
        database: true,
        db_latency_ms: 0,
        db_error: None,
        memory_used_mb: 0,
        memory_total_mb: 0,
        memory_percent: 0.0,
        overall: true,
    };

    let json = status.to_pretty_json();
    assert!(json.contains("line\\n\\\"quote\\\""));
}
