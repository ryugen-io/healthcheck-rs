use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

use super::url::HttpTarget;

pub fn perform_request(target: &HttpTarget, timeout: Duration) -> Result<(), String> {
    let addrs = resolve_addresses(target)?;
    let mut last_err = None;

    for addr in addrs {
        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(mut stream) => match handle_stream(&mut stream, target, timeout) {
                Ok(code) if (200..400).contains(&code) => return Ok(()),
                Ok(code) => return Err(format!("HTTP status {}", code)),
                Err(err) => {
                    last_err = Some(err);
                    continue;
                }
            },
            Err(err) => {
                last_err = Some(err.to_string());
                continue;
            }
        }
    }

    Err(last_err.unwrap_or_else(|| "failed to resolve HTTP host".into()))
}

fn handle_stream(
    stream: &mut TcpStream,
    target: &HttpTarget,
    timeout: Duration,
) -> Result<u16, String> {
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|err| err.to_string())?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|err| err.to_string())?;

    let request = build_request(target);
    stream
        .write_all(request.as_bytes())
        .map_err(|err| err.to_string())?;

    let mut reader = BufReader::new(stream);
    // Pre-allocate with reasonable capacity for typical HTTP status lines
    let mut status_line = String::with_capacity(64);
    reader
        .read_line(&mut status_line)
        .map_err(|_| "failed to read HTTP status line".to_string())?;

    if !status_line.starts_with("HTTP/") {
        return Err(format!("invalid HTTP response: {status_line}"));
    }

    status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| "missing HTTP status code".to_string())?
        .parse::<u16>()
        .map_err(|_| "malformed HTTP status code".to_string())
}

fn resolve_addresses(target: &HttpTarget) -> Result<Vec<SocketAddr>, String> {
    let address = if target.display_host.starts_with('[') {
        format!("{}:{}", target.display_host, target.port)
    } else {
        format!("{}:{}", target.host, target.port)
    };

    address
        .to_socket_addrs()
        .map(|iter| iter.collect())
        .map_err(|err| err.to_string())
}

fn build_request(target: &HttpTarget) -> String {
    // Pre-allocate capacity for the HTTP request
    // Typical size: "GET " (4) + path + " HTTP/1.1\r\n" (11) + "Host: " (6) + host + 
    // "\r\nUser-Agent: metamcp-healthcheck\r\nConnection: close\r\n\r\n" (60)
    let capacity = 81 + target.path.len() + target.display_host.len();
    let mut request = String::with_capacity(capacity);
    
    request.push_str("GET ");
    request.push_str(&target.path);
    request.push_str(" HTTP/1.1\r\nHost: ");
    request.push_str(&target.display_host);
    request.push_str("\r\nUser-Agent: metamcp-healthcheck\r\nConnection: close\r\n\r\n");
    
    request
}
