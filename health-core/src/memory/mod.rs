mod container;
mod host;

pub use container::parse_cgroup_bytes;
pub use host::parse_meminfo_value;

pub fn get_memory_stats() -> (u64, u64, f32) {
    if let Some(stats) = container::stats() {
        return stats;
    }

    if let Some(stats) = host::stats() {
        return stats;
    }

    (0, 0, 0.0)
}
