use std::process::{Command, Stdio};

#[test]
fn main() {
    let mut relay_handle = Command::new("target/debug/dnsl")
        .env("DNSL_LISTEN", "127.0.0.1:10000")
        .env("DNSL_LOOKUP", "0.0.0.0:10001")
        .env("DNSL_UPSTREAM", "8.8.8.8:53")
        .spawn()
        .unwrap();
    let query1 = Command::new("dig")
        .args(&["myl.moe", "@127.0.0.1", "-p", "10000"])
        .stdout(Stdio::piped())
        .status()
        .unwrap();
    assert!(query1.success());
    let query2 = Command::new("dig")
        .args(&["www.google.com", "@127.0.0.1", "-p", "10000"])
        .stdout(Stdio::piped())
        .status()
        .unwrap();
    assert!(query2.success());
    let query3 = Command::new("dig")
        .args(&["cname", "www.myl.moe", "@127.0.0.1", "-p", "10000"])
        .stdout(Stdio::piped())
        .status()
        .unwrap();
    assert!(query3.success());
    relay_handle.kill().unwrap();
}
