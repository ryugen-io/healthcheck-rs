use super::HealthStatus;

#[test]
fn health_status_to_json_captures_all_fields() {
    let status = HealthStatus {
        http: true,
        http_latency_ms: 321,
        http_error: Some("boom".into()),
        database: false,
        db_latency_ms: 42,
        db_error: None,
        memory_used_mb: 256,
        memory_total_mb: 512,
        memory_percent: 50.0,
        overall: false,
    };

    let json = status.to_pretty_json();

    assert_eq!(
        "{\n  \"http\": true,\n  \"http_latency_ms\": 321,\n  \"http_error\": \"boom\",\n  \"database\": false,\n  \"db_latency_ms\": 42,\n  \"db_error\": null,\n  \"memory_used_mb\": 256,\n  \"memory_total_mb\": 512,\n  \"memory_percent\": 50,\n  \"overall\": false\n}",
        json
    );
}
