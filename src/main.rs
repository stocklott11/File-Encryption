use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;

/// Represents what kind of action the user took.
#[derive(Debug, Clone, Copy)]
enum CryptoAction {
    Encrypt,
    Decrypt,
}

/// Stores a single history entry for this session.
#[derive(Debug, Clone)]
struct HistoryEntry {
    file_path: String,
    action: CryptoAction,
    success: bool,
}

/// Main application struct that holds the history.
struct FileCryptoApp {
    history: Vec<HistoryEntry>,
}

impl FileCryptoApp {
    /// Create a new instance of the app.
    fn new() -> Self {
        FileCryptoApp { history: Vec::new() }
    }

    /// Main loop that keeps the program running until the user quits.
    fn run(&mut self) {
        loop {
            println!();
            println!("================ File Encryptor ================");
            println!("1) Encrypt file");
            println!("2) Decrypt file");
            println!("3) Show history");
            println!("4) Quit");
            println!("=======================================================");
            print!("Enter your choice: ");
            flush_stdout();

            let choice = read_line_trimmed();

            match choice.as_str() {
                "1" => self.handle_encrypt(),
                "2" => self.handle_decrypt(),
                "3" => self.show_history(),
                "4" => {
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid choice. Please enter 1, 2, 3, or 4.");
                }
            }
        }
    }

    /// Handle the "Encrypt file" menu option.
    fn handle_encrypt(&mut self) {
        println!();
        println!("--- Encrypt File ---");
        print!("Enter input file path: ");
        flush_stdout();
        let input_path = read_line_trimmed();

        print!("Enter output file path (leave blank for default .enc): ");
        flush_stdout();
        let mut output_path = read_line_trimmed();
        if output_path.is_empty() {
            output_path = format!("{}.enc", input_path);
        }

        print!("Enter password: ");
        flush_stdout();
        let password = read_line_trimmed();

        let result = encrypt_file(&input_path, &output_path, &password);

        let success = result.is_ok();
        if let Err(e) = result {
            println!("Encryption failed: {}", e);
        } else {
            println!("File encrypted successfully to '{}'.", output_path);
        }

        self.add_history_entry(input_path, CryptoAction::Encrypt, success);
    }

    /// Handle the "Decrypt file" menu option.
    fn handle_decrypt(&mut self) {
        println!();
        println!("--- Decrypt File ---");
        print!("Enter input file path: ");
        flush_stdout();
        let input_path = read_line_trimmed();

        print!("Enter output file path (leave blank for default .dec): ");
        flush_stdout();
        let mut output_path = read_line_trimmed();
        if output_path.is_empty() {
            output_path = format!("{}.dec", input_path);
        }

        print!("Enter password: ");
        flush_stdout();
        let password = read_line_trimmed();

        let result = decrypt_file(&input_path, &output_path, &password);

        let success = result.is_ok();
        if let Err(e) = result {
            println!("Decryption failed: {}", e);
        } else {
            println!("File decrypted successfully to '{}'.", output_path);
        }

        self.add_history_entry(input_path, CryptoAction::Decrypt, success);
    }

    /// Add a new entry to the in memory history list.
    fn add_history_entry(&mut self, path: String, action: CryptoAction, success: bool) {
        let entry = HistoryEntry {
            file_path: path,
            action,
            success,
        };
        self.history.push(entry);
    }

    /// Display all history entries for this session and a small summary.
    fn show_history(&self) {
        println!();
        if self.history.is_empty() {
            println!("No history yet. Try encrypting or decrypting a file first.");
            return;
        }

        println!("--- History ---");
        for (index, entry) in self.history.iter().enumerate() {
            let action_str = match entry.action {
                CryptoAction::Encrypt => "Encrypt",
                CryptoAction::Decrypt => "Decrypt",
            };
            let status_str = if entry.success { "Success" } else { "Failed" };

            println!(
                "{}. [{}] {} -> {}",
                index + 1,
                action_str,
                entry.file_path,
                status_str
            );
        }

        let mut summary: HashMap<&str, usize> = HashMap::new();
        for entry in &self.history {
            let key = match entry.action {
                CryptoAction::Encrypt => "encrypt",
                CryptoAction::Decrypt => "decrypt",
            };
            *summary.entry(key).or_insert(0) += 1;
        }

        println!();
        println!("Summary this session:");
        println!(
            "Encrypted: {} file(s)",
            summary.get("encrypt").cloned().unwrap_or(0)
        );
        println!(
            "Decrypted: {} file(s)",
            summary.get("decrypt").cloned().unwrap_or(0)
        );
    }
}

/// Helper function that encrypts a file by reading it into memory,
/// running XOR over all bytes with a key derived from the password,
/// and writing out the result.
fn encrypt_file(input_path: &str, output_path: &str, password: &str) -> Result<(), String> {
    let data = fs::read(input_path).map_err(|e| format!("Failed to read input file: {}", e))?;
    let key_bytes = password.as_bytes();
    if key_bytes.is_empty() {
        return Err("Password cannot be empty.".to_string());
    }

    let encrypted = xor_with_key(&data, key_bytes);

    fs::write(output_path, encrypted)
        .map_err(|e| format!("Failed to write output file: {}", e))?;

    Ok(())
}

/// Helper function that decrypts a file. Since XOR is symmetric,
/// we can use the same operation for decryption.
fn decrypt_file(input_path: &str, output_path: &str, password: &str) -> Result<(), String> {
    let data = fs::read(input_path).map_err(|e| format!("Failed to read input file: {}", e))?;
    let key_bytes = password.as_bytes();
    if key_bytes.is_empty() {
        return Err("Password cannot be empty.".to_string());
    }

    let decrypted = xor_with_key(&data, key_bytes);

    fs::write(output_path, decrypted)
        .map_err(|e| format!("Failed to write output file: {}", e))?;

    Ok(())
}

/// Core XOR function that applies the key bytes repeatedly across the data.
fn xor_with_key(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, byte)| {
            let key_byte = key[i % key.len()];
            byte ^ key_byte
        })
        .collect()
}

/// Read a line from stdin, trim whitespace, and return it as a String.
fn read_line_trimmed() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line from stdin.");
    input.trim().to_string()
}

/// Ensure that printed prompts appear before the user types input.
fn flush_stdout() {
    io::stdout().flush().expect("Failed to flush stdout.");
}

/// Entry point of the program.
fn main() {
    println!("Welcome to the Rust File Encryptor.");
    println!("Note: This is a simple learning project and is not meant for real security.");
    let mut app = FileCryptoApp::new();
    app.run();
}