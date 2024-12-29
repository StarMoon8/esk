use aes_siv::aead::{generic_array::GenericArray, KeyInit};
use aes_siv::siv::Aes256Siv;
use anyhow::{anyhow, Context, Result};
use clap::{Arg, Command};
use rand::RngCore;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

const KEY_FILENAME: &str = "key.key";
const KEY_LENGTH: usize = 64;

fn main() -> Result<()> {
    let matches = Command::new("AES-SIV Encryption App")
        .version("1.0")
        .author("Your Name")
        .about("Encrypts and decrypts files using AES-SIV")
        .subcommand(
            Command::new("gen-key")
                .about("Generates a random encryption key")
                .arg(
                    Arg::new("key")
                        .help("Path to save the generated key")
                        .short('k')
                        .long("key")
                        .value_name("KEY_PATH"),
                ),
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a file")
                .arg(
                    Arg::new("input")
                        .help("Input file to encrypt")
                        .required(true)
                        .value_name("INPUT")
                        .index(1),
                )
                .arg(
                    Arg::new("output")
                        .help("Output file for encrypted data")
                        .required(true)
                        .value_name("OUTPUT")
                        .index(2),
                )
                .arg(
                    Arg::new("key")
                        .help("Path to the encryption key file")
                        .short('k')
                        .long("key")
                        .value_name("KEY_PATH"),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a file")
                .arg(
                    Arg::new("input")
                        .help("Input file to decrypt")
                        .required(true)
                        .value_name("INPUT")
                        .index(1),
                )
                .arg(
                    Arg::new("output")
                        .help("Output file for decrypted data")
                        .required(true)
                        .value_name("OUTPUT")
                        .index(2),
                )
                .arg(
                    Arg::new("key")
                        .help("Path to the encryption key file")
                        .short('k')
                        .long("key")
                        .value_name("KEY_PATH"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("gen-key", sub_m)) => {
            let key_path = sub_m
                .get_one::<String>("key")
                .map(|s| s.as_str())
                .unwrap_or(KEY_FILENAME);
            generate_key(KEY_LENGTH, key_path)
        }
        Some(("encrypt", sub_m)) => {
            let input = sub_m.get_one::<String>("input").unwrap();
            let output = sub_m.get_one::<String>("output").unwrap();
            let key_path = sub_m
                .get_one::<String>("key")
                .map(|s| s.as_str())
                .unwrap_or(KEY_FILENAME);
            let key = load_key(key_path)?;
            process_file(input, output, &key, true)
        }
        Some(("decrypt", sub_m)) => {
            let input = sub_m.get_one::<String>("input").unwrap();
            let output = sub_m.get_one::<String>("output").unwrap();
            let key_path = sub_m
                .get_one::<String>("key")
                .map(|s| s.as_str())
                .unwrap_or(KEY_FILENAME);
            let key = load_key(key_path)?;
            process_file(input, output, &key, false)
        }
        _ => {
            println!("Invalid subcommand. Use --help for usage.");
            Ok(())
        }
    }
}

fn generate_key(length: usize, key_path: &str) -> Result<()> {
    let mut key = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut key);
    let mut file = File::create(key_path)
        .with_context(|| format!("Failed to create key file at '{}'", key_path))?;
    file.write_all(&key)
        .with_context(|| "Failed to write key to file.")?;
    println!("Key generated and saved to '{}'", key_path);
    Ok(())
}

fn load_key(key_path: &str) -> Result<Vec<u8>> {
    if !Path::new(key_path).exists() {
        anyhow::bail!(
            "Key file '{}' not found. Please generate the key first.",
            key_path
        );
    }
    let mut key = vec![0u8; KEY_LENGTH];
    let mut file = File::open(key_path)
        .with_context(|| format!("Failed to open key file at '{}'", key_path))?;
    file.read_exact(&mut key)
        .with_context(|| "Failed to read key file.")?;
    Ok(key)
}

fn process_file(input: &str, output: &str, key: &[u8], encrypt: bool) -> Result<()> {
    let input_file = File::open(input)
        .with_context(|| format!("Failed to open input file '{}'.", input))?;
    let mut reader = BufReader::new(input_file);

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .with_context(|| format!("Failed to create output file '{}'.", output))?;
    let mut writer = BufWriter::new(output_file);

    let mut data = Vec::new();
    reader
        .read_to_end(&mut data)
        .with_context(|| "Failed to read input file.")?;

    // Initialize the cipher with the key
    let mut cipher = Aes256Siv::new(GenericArray::from_slice(key));

    let result = if encrypt {
        cipher
            .encrypt(&[&[]], &data)
            .map_err(|_| anyhow!("Encryption failed."))?
    } else {
        cipher
            .decrypt(&[&[]], &data)
            .map_err(|_| anyhow!("Decryption failed. Incorrect key or corrupted data."))?
    };

    writer
        .write_all(&result)
        .with_context(|| "Failed to write output file.")?;

    if encrypt {
        println!("Successfully encrypted '{}' to '{}'.", input, output);
    } else {
        println!("Successfully decrypted '{}' to '{}'.", input, output);
    }

    Ok(())
}