# Web server

[**Work in Progress**] - Experimental Rust software

This web server binds a socket address to a TCP listener and listens for
incoming TCP connections.

### Work in Progress

This repository contains experimental software that is under active development.
Features may be incomplete, and proper error handling may not to be fully
implemented.

## Usage

The crate can be executed by issuing `cargo run`.
```sh
cargo run
```

The web server accepts incoming requests at `localhost:1234`. Once the
connection is received, it sends a default `200 OK` HTTP response.
