mod writer;

use writer::JsonWriter;

pub struct HealthStatus {
    pub http: bool,
    pub http_latency_ms: u64,
    pub http_error: Option<String>,
    pub database: bool,
    pub db_latency_ms: u64,
    pub db_error: Option<String>,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_percent: f32,
    pub overall: bool,
}

impl HealthStatus {
    pub fn to_pretty_json(&self) -> String {
        let mut json = JsonWriter::new();
        json.bool("http", self.http);
        json.u64("http_latency_ms", self.http_latency_ms);
        json.str_opt("http_error", self.http_error.as_deref());
        json.bool("database", self.database);
        json.u64("db_latency_ms", self.db_latency_ms);
        json.str_opt("db_error", self.db_error.as_deref());
        json.u64("memory_used_mb", self.memory_used_mb);
        json.u64("memory_total_mb", self.memory_total_mb);
        json.f32("memory_percent", self.memory_percent);
        json.bool("overall", self.overall);
        json.finish()
    }
}
