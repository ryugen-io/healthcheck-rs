use std::{fs, path::Path};

const CGROUP_V2_USAGE: &str = "/sys/fs/cgroup/memory.current";
const CGROUP_V2_LIMIT: &str = "/sys/fs/cgroup/memory.max";
const CGROUP_V1_USAGE: &str = "/sys/fs/cgroup/memory/memory.usage_in_bytes";
const CGROUP_V1_LIMIT: &str = "/sys/fs/cgroup/memory/memory.limit_in_bytes";

pub fn stats() -> Option<(u64, u64, f32)> {
    if let Some(stats) = read_pair(CGROUP_V2_USAGE, CGROUP_V2_LIMIT) {
        return Some(stats);
    }

    read_pair(CGROUP_V1_USAGE, CGROUP_V1_LIMIT)
}

fn read_pair(usage: &str, limit: &str) -> Option<(u64, u64, f32)> {
    let usage_bytes = read_cgroup_bytes(Path::new(usage))?;
    let limit_bytes = read_cgroup_bytes(Path::new(limit))?;
    if limit_bytes == 0 {
        return None;
    }

    let used_mb = bytes_to_mb(usage_bytes);
    let total_mb = bytes_to_mb(limit_bytes).max(1);
    let percent = (usage_bytes as f32 / limit_bytes as f32) * 100.0;
    Some((used_mb, total_mb, percent))
}

fn read_cgroup_bytes(path: &Path) -> Option<u64> {
    let raw = fs::read_to_string(path).ok()?;
    parse_cgroup_bytes(&raw)
}

pub fn parse_cgroup_bytes(raw: &str) -> Option<u64> {
    let trimmed = raw.trim();
    if trimmed.eq_ignore_ascii_case("max") || trimmed.is_empty() {
        None
    } else {
        trimmed.parse::<u64>().ok()
    }
}

fn bytes_to_mb(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}
