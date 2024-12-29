
# Rust File Encryption GUI (Windows Testing App)

This is a simple Rust-based utility designed to serve as a starting point for developing and testing Windows applications. It provides a hybrid approach that bridges a basic graphical user interface (GUI) with a command-line interface (CLI) tool. This app allows users to select a file using a native file dialog and then passes that file path as an argument to a CLI tool (named `enc.exe` by default) for further processing.

The purpose of this project is to demonstrate early-stage Rust application development on Windows, where a basic GUI is integrated with a CLI executable. This can be useful for testing file processing commands like encryption, compression, or other file-related operations.

## Features

- **File Dialog Selection**: The app prompts the user to select a file via a native file dialog.
- **CLI Integration**: It passes the selected file as an argument to a CLI application (e.g., an encryption tool like `enc.exe`).
- **Windows Message Boxes**: The app provides user feedback via Windows message boxes, indicating success, error, or information.

## How It Works

1. The application starts by showing a native file dialog, allowing the user to select a file they want to process.
2. After the file is selected, the app assumes the presence of a CLI tool (`enc.exe`) in the same directory as the Rust executable.
3. The selected file path is passed as an argument to `enc.exe`.
4. The app then executes `enc.exe` and checks the exit status of the command:
   - If `enc.exe` runs successfully, the app displays a success message.
   - If `enc.exe` fails or the file isn't selected, appropriate error or informational messages are displayed.

## Prerequisites

### For Development:
- [Rust](https://www.rust-lang.org/tools/install) must be installed on your system.
- You need a CLI executable named `enc.exe` for testing. This can be any simple CLI program that accepts a file path as input and processes the file (such as encrypting, compressing, or any other operation).

### For Running:
- The Windows OS is required, as the app uses the Windows API for the message boxes and file dialog.

## Building and Running

1. Clone this repository:
    ```bash
    git clone <your-repo-url>
    cd <your-repo-folder>
    ```

2. Build the project with Cargo:
    ```bash
    cargo build --release
    ```

3. Create or rename your CLI application to `enc.exe` and place it in the same directory as the compiled GUI executable (`target/release/`).

4. Run the application:
    ```bash
    ./target/release/your_app_name.exe
    ```

## Customization

- **CLI Executable**: By default, the app expects `enc.exe` to be in the same folder as the GUI executable. You can modify the code to use a different CLI tool or a different name for the executable by changing the `enc.exe` reference in the code.
- **File Dialog Settings**: You can customize the file dialog to filter specific file types by adding options in the `rfd::FileDialog::new()` method.

## Example CLI App (`enc.exe`)

If you don't have a CLI app (`enc.exe`) to test with, you can create a simple Rust CLI app that echoes the file path:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: enc.exe <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("Processing file: {}", file_path);
}
```

Compile this as `enc.exe`, and place it alongside the GUI executable.

## Error Handling

- **File Not Selected**: If no file is selected, the app will display an informational message.
- **Missing `enc.exe`**: If `enc.exe` is not found in the application directory, the app will show an error message indicating the absence of the executable.
- **Failed Execution**: If `enc.exe` fails to run, the app will display an error message showing the failure reason.

## Future Enhancements

- Add more advanced error handling and logging features.
- Customize the file dialog to filter for specific file types.
- Extend functionality to allow interaction with more advanced CLI tools.
- Add additional command-line argument support for `enc.exe` to handle different modes or options.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

## Acknowledgments

This project makes use of the following crates:
- [`rfd`](https://crates.io/crates/rfd): For file dialogs.
- [`winapi`](https://crates.io/crates/winapi): For Windows API integration.
