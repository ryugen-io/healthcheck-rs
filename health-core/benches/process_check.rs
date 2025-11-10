use divan::Bencher;
use healthcheck_core::probes::process::ProcessCheck;
use std::collections::HashMap;

fn main() {
    divan::main();
}

#[divan::bench]
fn process_check_existing(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert("name".to_string(), "systemd".to_string());

    let check = ProcessCheck::from_params(&params).expect("valid params");

    bencher.bench(|| check.check());
}

#[divan::bench]
fn process_check_nonexistent(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert(
        "name".to_string(),
        "definitely_not_running_xxyyzz".to_string(),
    );

    let check = ProcessCheck::from_params(&params).expect("valid params");

    bencher.bench(|| check.check());
}

#[divan::bench]
fn process_config_parse(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert("name".to_string(), "systemd".to_string());

    bencher.bench(|| ProcessCheck::from_params(&params).unwrap());
}
