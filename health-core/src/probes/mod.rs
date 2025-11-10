pub mod database;
pub mod http;
pub mod process;
pub mod tcp;

#[derive(Clone)]
pub struct ProbeResult {
    pub ok: bool,
    pub latency_ms: u64,
    pub error: Option<String>,
}

impl ProbeResult {
    pub fn success(latency_ms: u64) -> Self {
        Self {
            ok: true,
            latency_ms,
            error: None,
        }
    }

    pub fn failure(latency_ms: u64, err: impl Into<String>) -> Self {
        Self {
            ok: false,
            latency_ms,
            error: Some(err.into()),
        }
    }
}

pub fn elapsed_ms(start: std::time::Instant) -> u64 {
    start.elapsed().as_millis() as u64
}
