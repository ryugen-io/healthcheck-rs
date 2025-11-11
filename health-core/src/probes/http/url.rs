#[derive(Debug)]
pub struct HttpTarget {
    pub host: String,
    pub display_host: String,
    pub port: u16,
    pub path: String,
}

pub fn parse_http_url(url: &str) -> Result<HttpTarget, String> {
    let lower = url.to_ascii_lowercase();
    if !lower.starts_with("http://") {
        return Err("only http:// URLs are supported".into());
    }

    let remainder = &url[7..];
    let mut parts = remainder.splitn(2, '/');
    let authority = parts.next().unwrap_or("");
    let path = match parts.next() {
        Some(p) if !p.is_empty() => format!("/{}", p),
        _ => "/".to_string(),
    };

    if authority.is_empty() {
        return Err("missing host".into());
    }

    if authority.starts_with('[') {
        parse_ipv6_authority(authority, path)
    } else {
        parse_host_authority(authority, path)
    }
}

fn parse_ipv6_authority(authority: &str, path: String) -> Result<HttpTarget, String> {
    let end = authority
        .find(']')
        .ok_or_else(|| "invalid IPv6 host".to_string())?;
    let host = authority[1..end].to_string();
    let mut port = 80u16;
    if authority.len() > end + 1 {
        if let Some(rest) = authority[end + 1..].strip_prefix(':') {
            port = rest
                .parse::<u16>()
                .map_err(|_| "invalid port".to_string())?;
        } else {
            return Err("invalid IPv6 authority".into());
        }
    }

    Ok(HttpTarget {
        display_host: format!("[{host}]"),
        host,
        port,
        path,
    })
}

fn parse_host_authority(authority: &str, path: String) -> Result<HttpTarget, String> {
    let (host_str, port) = if let Some(idx) = authority.rfind(':') {
        if authority[idx + 1..].chars().all(|c| c.is_ascii_digit()) {
            let parsed_port = authority[idx + 1..]
                .parse::<u16>()
                .map_err(|_| "invalid port".to_string())?;
            (&authority[..idx], parsed_port)
        } else {
            (authority, 80u16)
        }
    } else {
        (authority, 80u16)
    };

    if host_str.is_empty() {
        return Err("missing host".into());
    }

    let host = host_str.to_string();
    
    Ok(HttpTarget {
        host: host.clone(),
        display_host: host,
        port,
        path,
    })
}
