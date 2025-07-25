# Reverse shell client

[**Work in Progress**] - Experimental Rust software

This reverse shell client implementation establishes TCP connection to a target
(server) via `localhost:4444`, listens to incoming commands, executes them and
returns the command output to the server.

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
