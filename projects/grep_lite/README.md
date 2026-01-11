# grep-lite

A simplified version of the `grep` command-line tool, implemented in Rust.

## Functionality

This program takes a search pattern and a file path as input. It then searches the file line by line and prints any line that contains the search pattern.

## Usage

```bash
cargo run <pattern> <file_path>
```

For example:

```bash
cargo run "hello" my_file.txt
```

This will search for the string "hello" in the file `my_file.txt` and print any matching lines.

## Building

To build the project, simply run:

```bash
cargo build
```

## Running Tests

To run the tests, use the command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.
