use std::fs;

pub fn stats() -> Option<(u64, u64, f32)> {
    let meminfo = fs::read_to_string("/proc/meminfo").ok()?;
    let mut total_kb = None;
    let mut available_kb = None;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total_kb = parse_meminfo_value(line);
        } else if line.starts_with("MemAvailable:") {
            available_kb = parse_meminfo_value(line);
        }

        if total_kb.is_some() && available_kb.is_some() {
            break;
        }
    }

    let total_kb = total_kb?;
    let available_kb = available_kb.unwrap_or(0);
    if total_kb == 0 || available_kb > total_kb {
        return None;
    }

    let used_kb = total_kb - available_kb;
    let used_mb = used_kb / 1024;
    let total_mb = total_kb / 1024;
    let percent = if total_kb == 0 {
        0.0
    } else {
        (used_kb as f32 / total_kb as f32) * 100.0
    };

    Some((used_mb, total_mb, percent))
}

pub fn parse_meminfo_value(line: &str) -> Option<u64> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    parts[1].parse::<u64>().ok()
}
