AOTP: OTP XOR Encryption/Decryption CLI Tool
AOTP is a command-line application written in Rust that performs encryption and decryption using the One-Time Pad (OTP) method with XOR operations. It reads an input file and a key file, applies the XOR operation between them, and writes the result to an output file. This tool is designed for simplicity, efficiency, and reliability, utilizing parallel processing to handle large files efficiently.

Features
This application offers several features to enhance performance and usability. It dynamically determines the optimal buffer size based on system memory and file size, ensuring efficient memory usage. The tool supports chunk-based parallel processing to speed up the XOR operations on large datasets. It also includes secure file handling practices, such as setting appropriate file permissions and securely zeroizing buffers after use to enhance security. Additionally, it provides a progress bar to keep users informed about the processing status and includes detailed logging for easier troubleshooting.

Installation
To use this tool, you need to have Rust and Cargo installed on your system. Clone the repository or copy the source code into a directory. Navigate to the project directory in your terminal and run cargo build --release to compile the application. This will create an executable in the target/release directory.

Usage
The application is executed from the command line and requires three positional arguments: the input file path, the key file path, and the output file path. It also accepts optional flags for buffer size and overwriting existing output files. The basic usage is as follows:

sh
Copy code
./aotp <input_file> <key_file> <output_file>
You can specify the buffer size in megabytes using the -b or --buffer-size option. To overwrite an existing output file, use the -f or --force flag. Here is an example with optional arguments:

sh
Copy code
./aotp input.txt key.bin output.enc -b 16 -f
This command processes input.txt using key.bin, writes the encrypted data to output.enc, uses a buffer size of 16 megabytes, and overwrites the output file if it already exists.

Important Considerations
Ensure that the key file is at least as large as the input file, as the OTP method requires a key that matches or exceeds the size of the data being processed. If the key file is shorter than the input file, the program will terminate with an error. Additionally, be cautious with key management and secure deletion practices to maintain the confidentiality of your data.

Logging and Progress Monitoring
The tool provides informative logging messages that can help you understand its operation and diagnose any issues. It also features a progress bar that displays the elapsed time, data processed, total data, and estimated time remaining, giving you real-time feedback on the processing status.

Security Features
AOTP incorporates security measures to protect sensitive data during processing. It securely zeroizes buffers after they are no longer needed, reducing the risk of data remnants remaining in memory. On Unix systems, it sets strict file permissions for the output file to prevent unauthorized access.

License
This application is open-source and available under the MIT License. You are free to use, modify, and distribute it in accordance with the license terms.

