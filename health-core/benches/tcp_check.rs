use divan::Bencher;
use healthcheck_core::probes::tcp::TcpCheck;
use std::collections::HashMap;

fn main() {
    divan::main();
}

#[divan::bench]
fn tcp_check_localhost(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert("host".to_string(), "127.0.0.1".to_string());
    params.insert("port".to_string(), "22".to_string());
    params.insert("timeout_ms".to_string(), "1000".to_string());

    let check = TcpCheck::from_params(&params).expect("valid params");

    bencher.bench(|| check.check());
}

#[divan::bench]
fn tcp_config_parse(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert("host".to_string(), "127.0.0.1".to_string());
    params.insert("port".to_string(), "22".to_string());
    params.insert("timeout_ms".to_string(), "1000".to_string());

    bencher.bench(|| TcpCheck::from_params(&params).unwrap());
}
