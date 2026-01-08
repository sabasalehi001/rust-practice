# CLI Argument Parser

A simple command-line argument parser written in Rust.

## Usage

Run the program with different arguments to see the effect.

```bash
./cli_arg_parser --help
./cli_arg_parser --name John --age 30
./cli_arg_parser --verbose
```

## Building

```bash
cargo build
```

## Running

```bash
cargo run -- --help
cargo run -- --name YourName --age YourAge
```