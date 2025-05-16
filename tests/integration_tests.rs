use std::io::Write;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_version_flag() {
    // Run with --version flag
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check version output
    assert!(stdout.contains("Fractal Amadeus v"));
    assert!(stdout.contains("A proof-of-alignment node")); // Fixed: match the actual description from Cargo.toml
    assert!(stdout.contains("Authors:"));
}

#[test]
fn test_help_command() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Send the help command
    writeln!(stdin, "help").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit the application
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check help output
    assert!(stdout.contains("Available commands:"));
    assert!(stdout.contains("help"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("get"));
    assert!(stdout.contains("exit"));
}

#[test]
fn test_add_and_get_concept() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Add a concept
    writeln!(stdin, "add consciousness_IIT|integrated information above threshold Φ|Integrated Information Theory|0.8")
        .expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Get the concept
    writeln!(stdin, "get consciousness_IIT").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check output
    assert!(stdout.contains("Concept 'consciousness_IIT' added to memory graph."));
    assert!(stdout.contains("Definition: integrated information above threshold Φ"));
    assert!(stdout.contains("Framework: Integrated Information Theory"));
    assert!(stdout.contains("Confidence: 0.80"));
}

#[test]
fn test_invalid_add_command() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Invalid add command (missing parameters)
    writeln!(stdin, "add").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check error message
    assert!(stdout.contains("Usage: add <concept_key>|<definition>|<framework>|<confidence>"));
}

#[test]
fn test_invalid_add_format() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Invalid add format (not enough parts)
    writeln!(stdin, "add key|definition").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check error message
    assert!(stdout.contains(
        "Please provide concept in format: concept_key|definition|framework|confidence(optional)"
    ));
}

#[test]
fn test_invalid_get_command() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Invalid get command (missing parameters)
    writeln!(stdin, "get").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check error message
    assert!(stdout.contains("Usage: get <concept_key>"));
}

#[test]
fn test_get_nonexistent_concept() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Get a concept that doesn't exist
    writeln!(stdin, "get nonexistent_concept").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check error message
    assert!(stdout.contains("Concept 'nonexistent_concept' not found in memory graph."));
}

#[test]
fn test_unknown_command() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Unknown command
    writeln!(stdin, "unknown_command").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check error message
    assert!(stdout.contains("Unknown command. Type 'help' for available commands."));
}

#[test]
fn test_version_command() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Version command
    writeln!(stdin, "version").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check version output
    assert!(stdout.contains("Fractal Amadeus v"));
    assert!(stdout.contains("Authors:"));
}

#[test]
fn test_empty_input() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Empty input (just press Enter)
    writeln!(stdin, "").expect("Failed to write to stdin");
    sleep(Duration::from_millis(100)); // Give time for processing

    // Exit
    writeln!(stdin, "exit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // The program should prompt for input again without error
    assert!(!stdout.contains("error"));
    assert!(stdout.contains("Exiting Fractal Amadeus"));
}

#[test]
fn test_quit_command() {
    // Start the application
    let mut child = Command::new("cargo")
        .args(&["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    // Use quit instead of exit
    writeln!(stdin, "quit").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check exit message
    assert!(stdout.contains("Exiting Fractal Amadeus"));
}
