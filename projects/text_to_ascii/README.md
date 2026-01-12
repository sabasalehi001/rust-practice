# Text to ASCII Art Generator

This simple Rust project takes text input and converts it into ASCII art using a predefined font.

## Usage

1.  Clone the repository.
2.  Build the project using `cargo build --release`.
3.  Run the executable located in `target/release/text_to_ascii` followed by the text you want to convert.

   ```bash
   ./target/release/text_to_ascii "Hello World"
   ```

## Example Output

For the input "Hello", the output might look like this:

```
  _   _       _   
 | | | |     | |
 | |_| | __ _| |_ _   _ 
 |  _  |/ _` | __| | | |
 | | | | (_| | |_| |_| |
 |_| |_|\__,_|\__|\__, |
                     __/ |
                    |___/
```

## Customization

The font used for the ASCII art is hardcoded in `src/main.rs`. You can modify this to use a different font or create your own.

## License

MIT