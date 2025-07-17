// Imports
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

use aes_gcm::Nonce;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit, OsRng},
};
use pbkdf2::password_hash::SaltString;

fn main() {
    // Generate salt
    let salt = SaltString::generate(&mut OsRng);
    println!("salt {:?}", salt);

    // Get `HOME` directory
    let mut path = if cfg!(target_os = "windows") {
        env::var_os("USERPROFILE").map(PathBuf::from)
    } else {
        env::var_os("HOME").map(PathBuf::from)
    }
    .expect(":: [-] :: Home directory");

    // Push `target/` directory to avoid harm
    path.push("target");

    // Discover files
    let files = discover_dirs(&path).unwrap_or_else(|err| {
        eprintln!(":: [-] :: {}", err);
        process::exit(1);
    });

    // Encryption key file
    let key = fs::read("encryption.key").expect(":: [-] :: Cannot read encryption key file");

    // Iterate over files
    for file in files {
        println!(":: [i] :: File :: {:?}", file);

        // Read file
        let encrypted_data = match fs::read(&file) {
            Ok(data) => data,
            Err(err) => {
                eprintln!(":: [-] :: {}", err);
                return;
            }
        };

        // Retrieve nonce bytes & ciphertext bytes
        let nonce_bytes = &encrypted_data[0..12];
        let ciphertext = &encrypted_data[12..];

        // Get nonce bytes
        let nonce = Nonce::from_slice(nonce_bytes);

        // Convert key to GenericArray
        let key = GenericArray::from_slice(&key);

        // Aes256Gcm - Initialize instance
        let cipher = Aes256Gcm::new(key);

        // Decrypt ciphertext
        let plain_text = cipher
            .decrypt(nonce, ciphertext)
            .expect(":: [-] :: Failed to decrypt");
        println!(
            ":: Plain text :: {:?}",
            String::from_utf8_lossy(&plain_text)
        );
    }
}

// Discover directories
fn discover_dirs(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let mut sub_files = discover_dirs(&path)?;
            files.append(&mut sub_files);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}
