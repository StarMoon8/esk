#![windows_subsystem = "windows"] // Prevents a console window from appearing

use rfd::FileDialog;
use std::process::Command;
use std::path::PathBuf;
use std::io;

// Import necessary items from winapi for MessageBox
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{MessageBoxW, MB_OK};

fn main() -> io::Result<()> {
    // Open the file dialog for the user to select a file
    let selected_file: Option<PathBuf> = FileDialog::new()
        .set_title("Select a file to encrypt")
        .pick_file();

    match selected_file {
        Some(path) => {
            // Determine the path to enc.exe
            // Assuming enc.exe is in the same directory as the GUI executable
            let exe_path = std::env::current_exe()?;
            let exe_dir = exe_path.parent().expect("Failed to get executable directory");
            let enc_path = exe_dir.join("enc.exe");

            if !enc_path.exists() {
                show_message("Error", "enc.exe not found in the application directory.");
                return Ok(());
            }

            // Execute enc.exe with the selected file as an argument
            let status = Command::new(enc_path)
                .arg(path.to_str().unwrap())
                .status();

            match status {
                Ok(status) => {
                    if status.success() {
                        show_message("Success", "File encrypted successfully.");
                    } else {
                        show_message("Error", "Failed to encrypt the file.");
                    }
                }
                Err(e) => {
                    show_message("Error", &format!("Failed to execute enc.exe: {}", e));
                }
            }
        }
        None => {
            // User canceled the file selection
            show_message("Info", "No file was selected.");
        }
    }

    Ok(())
}

// Function to display message boxes using the Windows API
fn show_message(title: &str, message: &str) {
    // Convert Rust strings to wide strings (UTF-16) for Windows API
    let title_wide: Vec<u16> = OsStr::new(title)
        .encode_wide()
        .chain(std::iter::once(0)) // Null-terminate the string
        .collect();
    let message_wide: Vec<u16> = OsStr::new(message)
        .encode_wide()
        .chain(std::iter::once(0)) // Null-terminate the string
        .collect();

    unsafe {
        // Call the Windows API MessageBoxW function
        MessageBoxW(
            std::ptr::null_mut(),          // No owner window
            message_wide.as_ptr(),         // Message text
            title_wide.as_ptr(),           // Message box title
            MB_OK,                          // Message box type (OK button)
        );
    }
}
