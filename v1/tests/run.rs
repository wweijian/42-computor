#![allow(dead_code)]

use std::process::Command;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(5);

pub fn run(input: &str) -> std::process::Output {
    let mut child = Command::new("cargo")
        .args(["run", "--", input])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    let start = std::time::Instant::now();
    loop {
        match child.try_wait().expect("failed to wait") {
            Some(_) => break,
            None if start.elapsed() >= TIMEOUT => {
                child.kill().ok();
                panic!("timed out after {:?} for input {:?}", TIMEOUT, input);
            }
            None => std::thread::sleep(Duration::from_millis(50)),
        }
    }

    child.wait_with_output().expect("failed to collect output")
}

pub fn ok(input: &str) {
    let output = run(input);
    assert!(
        output.status.success(),
        "expected success for {:?}, got stderr: {}",
        input,
        String::from_utf8_lossy(&output.stderr)
    );
}

pub fn fail(input: &str) {
    let output = run(input);
    assert!(
        !output.status.success(),
        "expected failure for {:?}, but it succeeded\n{}",
        input,
        String::from_utf8_lossy(&output.stderr)
    );
}

pub fn stdout_contains(input: &str, expected: &str) {
    let output = run(input);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(expected),
        "expected stdout to contain {:?} for input {:?}, got:\n{}",
        expected,
        input,
        stdout
    );
}
