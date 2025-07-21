# Rust

[**Work in Progress**] - Experimental Rust software

## Disclaimer

This repository contains malware samples intended solely for **educational and
research purposes**. I am not responsible for any misuse or damage resulting
from the use of this code. By accessing this repository, you agree to comply
with all relevant laws and ethical standards.

**Do not use this software for malicious activities.**

### Work in Progress

This repository contains experimental software that is under active development.
Features may be incomplete, and proper error handling may not to be fully
implemented.

## Usage

### Execution

### Target

The target is set to `~/home` to avoid damaging the contents of the `HOME`
directory.
```sh
# UNIX
/home/{user}/.home
```

```cmd
# Windows
C:\Users\<user>\.home
```

### Encryption key

The encryption key file (`encryption.key`) is read from the crate root.
