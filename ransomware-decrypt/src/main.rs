use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use aes_gcm::Nonce;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit},
};

fn main() {
    // Get `HOME` directory
    let mut path = if cfg!(target_os = "windows") {
        env::var_os("USERPROFILE").map(PathBuf::from)
    } else {
        env::var_os("HOME").map(PathBuf::from)
    }
    .unwrap_or_else(|| {
        eprintln!(":: [-] :: Could not determine home directory");
        std::process::exit(1);
    });

    // Append `.home/` (set to avoid encrypting `HOME`)
    path.push(".home");

    // Discover files
    let files = discover_files(&path).unwrap_or_else(|err| {
        eprintln!(":: [-] :: {}", err);
        process::exit(1);
    });

    // Read encryption key file
    let key = fs::read("encryption.key").expect(":: [-] :: Cannot read encryption key file");

    // Iterate over files
    for file in files {
        // Read file contents
        let encrypted_data = match fs::read(&file) {
            Ok(data) => data,
            Err(err) => {
                eprintln!(":: [-] :: {}", err);
                return;
            }
        };

        // Retrieve nonce & ciphertext bytes
        let nonce_bytes = &encrypted_data[0..12];
        let ciphertext = &encrypted_data[12..];

        // Get nonce bytes
        let nonce = Nonce::from_slice(nonce_bytes);

        // Convert key to GenericArray
        let key = GenericArray::from_slice(&key);

        // Initialize instance (Aes256Gcm)
        let cipher = Aes256Gcm::new(key);

        // Decrypt ciphertext
        let plain_text = cipher
            .decrypt(nonce, ciphertext)
            .expect(":: [-] :: Decrypt ciphertext");

        // Create plain text file
        let mut out_path = file.clone();
        out_path.set_extension("");

        // Rename original file
        fs::rename(&file, &out_path).expect(":: [-] :: Rename original file");

        // Overwrite renamed file
        let mut file = File::create(&out_path).expect(":: [-] :: Create file");

        // Write `plain text` to file
        file.write_all(&plain_text)
            .expect(":: [-] :: Write plain text");

        println!(":: [+] :: Decrypted file ::  {:?}", out_path);
    }
}

// Discover files
fn discover_files(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let mut sub_files = discover_files(&path)?;
            files.append(&mut sub_files);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}
