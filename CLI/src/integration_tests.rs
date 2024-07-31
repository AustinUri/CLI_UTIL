use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn get_binary_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("target");
    path.push("debug");
    path.push("CLI");
    path
}

#[test]
fn test_command_line() {
    let binary_path = get_binary_path();
    let child = Command::new(binary_path)
        .arg("echo")
        .arg("Hello, world!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let mut stdout = String::new();
    child.stdout.unwrap().read_to_string(&mut stdout).unwrap();
    assert!(stdout.contains("Hello, world!"));
}

#[test]
fn test_help_command() {
    let binary_path = get_binary_path();
    let output = Command::new(binary_path)
        .arg("help")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Available commands:"));
}
