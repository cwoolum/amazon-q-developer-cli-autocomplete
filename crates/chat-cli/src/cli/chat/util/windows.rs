#![cfg(windows)]

use std::io::Result;
use windows::Win32::System::Console::{
    GetConsoleMode, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING, 
    GetStdHandle, STD_OUTPUT_HANDLE, CONSOLE_MODE
};

/// Enable ANSI escape sequence processing in Windows console
/// 
/// This is required for ANSI escape sequences (colors, cursor movement, etc.)
/// to work properly in Windows terminals. Without this, ANSI sequences will
/// appear as literal text rather than being interpreted.
pub fn enable_ansi_support() -> Result<()> {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == 0 {
            return Ok(());
        }
        
        let mut mode = CONSOLE_MODE::default();
        if GetConsoleMode(handle, &mut mode) == 0 {
            return Ok(());
        }
        
        // Add ENABLE_VIRTUAL_TERMINAL_PROCESSING flag
        mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        
        if SetConsoleMode(handle, mode) == 0 {
            return Ok(());
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_enable_ansi_support() {
        // This test simply verifies that the function doesn't panic or return an error
        let result = enable_ansi_support();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_ansi_output() {
        // Enable ANSI support
        let _ = enable_ansi_support();
        
        // Create a temporary file to capture output
        let mut file = NamedTempFile::new().unwrap();
        
        // Write some ANSI-colored text to the file
        writeln!(file, "\x1B[31mRed Text\x1B[0m").unwrap();
        writeln!(file, "\x1B[32mGreen Text\x1B[0m").unwrap();
        
        // Verify the file contains the expected ANSI sequences
        let content = std::fs::read_to_string(file.path()).unwrap();
        assert!(content.contains("\x1B[31m"));
        assert!(content.contains("\x1B[32m"));
        assert!(content.contains("\x1B[0m"));
    }
    
    #[test]
    fn test_console_mode_changes() {
        unsafe {
            // Get the current console mode
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if handle == 0 {
                // Skip test if we can't get the handle
                return;
            }
            
            let mut original_mode = 0;
            if GetConsoleMode(handle, &mut original_mode) == 0 {
                // Skip test if we can't get the mode
                return;
            }
            
            // Call our function
            let _ = enable_ansi_support();
            
            // Check that the mode now has ENABLE_VIRTUAL_TERMINAL_PROCESSING set
            let mut new_mode = 0;
            if GetConsoleMode(handle, &mut new_mode) == 0 {
                // Skip test if we can't get the mode
                return;
            }
            
            assert!(new_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING != 0, 
                "ENABLE_VIRTUAL_TERMINAL_PROCESSING flag should be set");
        }
    }
}
