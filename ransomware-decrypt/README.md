# Ransomware decryptor

[**Work in Progress**] - Experimental Rust software

This ransomware decryptor implementation iterates over the encrypted files in
the `target` directory and decrypts them with the encryption key.

## Disclaimer

This repository contains a malware implementation intended solely for
**educational and learning purposes**. I am not responsible for any misuse or
damage resulting from the use of this code. By accessing this repository, you
agree to comply with all relevant laws and ethical standards.

**Do not use this software for malicious activities!**

### Work in Progress

This repository contains experimental software that is under active development.
Features may be incomplete, and proper error handling may not to be fully
implemented.

## Usage

### Execution

The crate can be executed by issuing `cargo run`.
```sh
cargo run
```

### Target directory

The target is set to `~/.home` to avoid damaging the contents of the `HOME`
directory.
```sh
# UNIX
/home/<user>/.home
```

```cmd
# Windows
C:\Users\<user>\.home
```

### Encryption key

The encryption key file (`encryption.key`) is read from the crate root.
