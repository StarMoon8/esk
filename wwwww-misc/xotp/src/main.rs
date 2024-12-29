use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = get_input("Enter the path to the input file: ")?;
    let key_path = get_input("Enter the path to the key file: ")?;
    let output_path = get_input("Enter the path to the output file: ")?;

    let input_data = read_file_to_bytes(&input_path)?;
    let key_data = read_file_to_bytes(&key_path)?;
    let output_data = process_data(&input_data, &key_data)?;
    write_output_file(&output_path, &output_data)?;

    println!("Operation completed successfully.");

    Ok(())
}

/// Prompt the user for input and return the trimmed string.
fn get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?; // Ensure the prompt is displayed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

/// Read the entire contents of a file into a byte vector.
fn read_file_to_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

/// Write the byte vector to the output file.
fn write_output_file(path: &str, data: &[u8]) -> Result<(), io::Error> {
    let mut output_file = File::create(path)?;
    output_file.write_all(data)?;
    Ok(())
}

/// Process the data using the key with a one-time pad cipher.
fn process_data(input_data: &[u8], key_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if key_data.is_empty() {
        return Err("Key file cannot be empty".into());
    }
    if key_data.len() < input_data.len() {
        return Err("Key must be at least as long as the input data".into());
    }
    let output_data: Vec<u8> = input_data
        .iter()
        .zip(key_data.iter())
        .map(|(&byte, &key_byte)| byte ^ key_byte)
        .collect();
    Ok(output_data)
}
