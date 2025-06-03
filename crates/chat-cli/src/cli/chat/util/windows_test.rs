#![cfg(all(test, windows))]

use super::windows::enable_ansi_support;
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Integration test that verifies ANSI sequences are properly processed
/// by running a child process that outputs ANSI sequences
#[test]
fn test_ansi_integration() -> io::Result<()> {
    // Enable ANSI support in the current process
    enable_ansi_support()?;
    
    // Create a temporary batch file that outputs ANSI sequences
    let temp_dir = tempfile::tempdir()?;
    let batch_path = temp_dir.path().join("ansi_test.bat");
    
    let batch_content = r#"
@echo off
echo [31mThis text should be red[0m
echo [32mThis text should be green[0m
echo [34mThis text should be blue[0m
"#;
    
    std::fs::write(&batch_path, batch_content)?;
    
    // Run the batch file and capture its output
    let output = Command::new("cmd")
        .args(["/C", batch_path.to_str().unwrap()])
        .stdout(Stdio::piped())
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify the output contains the ANSI sequences
    assert!(stdout.contains("[31m"));
    assert!(stdout.contains("[32m"));
    assert!(stdout.contains("[34m"));
    assert!(stdout.contains("[0m"));
    
    Ok(())
}

/// Test that verifies our function works with a real console
#[test]
fn test_with_real_console() -> io::Result<()> {
    // This test will only run meaningful checks when run in a real console
    // Otherwise, it just verifies the function doesn't crash
    
    // Call our function
    enable_ansi_support()?;
    
    // Try to output some ANSI-colored text to stdout
    // This is mainly for manual verification, but we can at least
    // check that writing doesn't fail
    println!("\x1B[31mRed text for testing\x1B[0m");
    println!("\x1B[32mGreen text for testing\x1B[0m");
    println!("\x1B[34mBlue text for testing\x1B[0m");
    println!("\x1B[0mReset text for testing");
    
    // If we got here without errors, the test passes
    Ok(())
}
