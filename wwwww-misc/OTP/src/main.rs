use clap::Parser;
use sysinfo::System; // Removed SystemExt from here
use zeroize::Zeroize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info};
use env_logger;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use rayon::prelude::*;

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

// Removed unused import for Windows
// #[cfg(windows)]
// use std::os::windows::fs::OpenOptionsExt;

/// Constants for buffer sizes (in bytes)
const MIN_BUFFER_SIZE: usize = 4 * 1024 * 1024;    // 4 MB
const MAX_BUFFER_SIZE: usize = 64 * 1024 * 1024;   // 64 MB

/// A simple OTP XOR Encryption/Decryption CLI tool.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path
    input: String,

    /// Key file path
    key: String,

    /// Output file path
    output: String,

    /// Buffer size in megabytes (optional)
    #[arg(short, long)]
    buffer_size: Option<usize>,

    /// Overwrite output file if it exists
    #[arg(short, long)]
    force: bool,
}

/// Determines the optimal buffer size based on system memory and file size.
/// Ensures the buffer size is within defined minimum and maximum limits.
fn determine_buffer_size(system: &System, file_size: u64) -> usize {
    let available_memory_bytes = system.available_memory(); // Now directly on System

    // Use 1/20th of available memory or MAX_BUFFER_SIZE, whichever is smaller
    let buffer_based_on_mem = (available_memory_bytes / 20)
        .min(MAX_BUFFER_SIZE as u64) as usize;

    // Determine buffer size based on file size
    let buffer_based_on_file = if file_size > 10 * 1024 * 1024 * 1024 {
        32 * 1024 * 1024 // 32 MB for files > 10 GB
    } else if file_size > 1 * 1024 * 1024 * 1024 {
        16 * 1024 * 1024 // 16 MB for files > 1 GB
    } else {
        8 * 1024 * 1024 // 8 MB for smaller files
    };

    // Choose the smaller buffer size to prevent excessive memory usage
    let buffer_size = buffer_based_on_mem.min(buffer_based_on_file);

    // Clamp the buffer size within defined limits
    buffer_size.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE)
}

/// Processes a chunk of data by performing XOR between input and key buffers.
/// Utilizes chunk-based parallelism for optimized performance.
fn process_chunk(input: &[u8], key: &[u8], output: &mut [u8]) {
    let chunk_size = 1 * 1024 * 1024; // 1 MB

    // Ensure that input, key, and output have the same length
    assert_eq!(input.len(), key.len());
    assert_eq!(input.len(), output.len());

    output
        .par_chunks_mut(chunk_size)
        .zip(input.par_chunks(chunk_size))
        .zip(key.par_chunks(chunk_size))
        .for_each(|((output_chunk, input_chunk), key_chunk)| {
            for i in 0..input_chunk.len() {
                output_chunk[i] = input_chunk[i] ^ key_chunk[i];
            }
        });
}

fn main() -> std::io::Result<()> {
    // Initialize the logger with environment variable support
    env_logger::init();

    info!("Starting OTP XOR Encryption/Decryption.");

    // Parse command-line arguments
    let args = Args::parse();

    // Initialize system information
    let mut sys = System::new_all();
    sys.refresh_memory();

    // Get input file size
    let input_metadata = std::fs::metadata(&args.input)?;
    let input_size = input_metadata.len();

    // Determine buffer size: user-specified or dynamic based on system and file size
    let buffer_size = if let Some(mb) = args.buffer_size {
        let buffer = mb * 1024 * 1024;
        let clamped = buffer.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE);
        info!("Buffer size specified by user: {} MB", clamped / (1024 * 1024));
        println!("Buffer size used: {} MB", clamped / (1024 * 1024));
        clamped
    } else {
        let buffer_size = determine_buffer_size(&sys, input_size);
        info!(
            "Dynamically determined buffer size: {} MB based on system resources and file size.",
            buffer_size / (1024 * 1024)
        );
        println!("Buffer size used: {} MB", buffer_size / (1024 * 1024));
        buffer_size
    };

    debug!("Using buffer size: {} bytes", buffer_size);

    // Check if output file already exists to prevent overwriting
    if Path::new(&args.output).exists() && !args.force {
        error!(
            "Output file '{}' already exists. Use --force to overwrite.",
            args.output
        );
        std::process::exit(1);
    }

    // Check if input and key files exist
    if !Path::new(&args.input).is_file() {
        error!(
            "Input file '{}' does not exist or is not a file.",
            args.input
        );
        std::process::exit(1);
    }

    if !Path::new(&args.key).is_file() {
        error!(
            "Key file '{}' does not exist or is not a file.",
            args.key
        );
        std::process::exit(1);
    }

    // Get key file size
    let key_metadata = std::fs::metadata(&args.key)?;
    let key_size = key_metadata.len();

    info!(
        "Input file size: {} bytes ({:.2} GB)",
        input_size,
        input_size as f64 / 1_073_741_824.0
    );
    info!(
        "Key file size: {} bytes ({:.2} GB)",
        key_size,
        key_size as f64 / 1_073_741_824.0
    );

    if key_size < input_size {
        error!(
            "Key file '{}' is smaller ({}) than input file '{}' ({}).",
            args.key, key_size, args.input, input_size
        );
        std::process::exit(1);
    }

    // Open input and key files
    let input_file = File::open(&args.input)?;
    let key_file = File::open(&args.key)?;

    // Open output file with secure permissions (rw-------)
    #[cfg(unix)]
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600) // Set permissions to rw-------
        .open(&args.output)?;

    #[cfg(not(unix))]
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&args.output)?;

    // Create buffered readers and writer with configurable buffer size
    let mut input_reader = BufReader::with_capacity(buffer_size, input_file);
    let mut key_reader = BufReader::with_capacity(buffer_size, key_file);
    let mut writer = BufWriter::with_capacity(buffer_size, output_file);

    // Initialize progress bar
    let pb = ProgressBar::new(input_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Start processing loop
    loop {
        // Allocate buffers
        let mut input_buffer = vec![0u8; buffer_size];
        let mut key_buffer = vec![0u8; buffer_size];
        let mut output_buffer = vec![0u8; buffer_size];

        // Read a chunk from input file
        let input_bytes_read = input_reader.read(&mut input_buffer)?;
        if input_bytes_read == 0 {
            break; // EOF
        }

        // Resize buffers to actual bytes read
        input_buffer.truncate(input_bytes_read);
        key_buffer.truncate(input_bytes_read);
        output_buffer.truncate(input_bytes_read);

        // Read the corresponding chunk from key file
        key_reader.read_exact(&mut key_buffer)?;

        // Perform XOR operation
        process_chunk(&input_buffer, &key_buffer, &mut output_buffer);

        // Write the processed chunk to output file
        writer.write_all(&output_buffer)?;

        // Update progress bar
        pb.inc(input_bytes_read as u64);

        // Zero out the buffers securely
        input_buffer.zeroize();
        key_buffer.zeroize();
        output_buffer.zeroize();
    }

    // Flush the writer to ensure all data is written
    writer.flush()?;

    // Finish progress bar
    pb.finish_with_message("Processing complete.");

    info!("Encryption/Decryption completed successfully.");

    Ok(())
}
