use std::env;

// Simple hardcoded font for demonstration
const FONT: [&str; 5] = [
    " _   _       _   ",
    "| | | |     | |",
    "| |_| | __ _| |_ _   _ ",
    "|  _  |/ _` | __| | | |",
    "| | | | (_| | |_| |_| |",
    "|_| |_|\\__,_|\\__|\\__, |",
    "                    __/ |",
    "                   |___/"
];

fn char_to_ascii(c: char) -> Option<[&'static str; 5]> {
    match c.to_ascii_uppercase() {
        'H' => Some([
            " _   _",
            "| | | |",
            "| |_| |",
            "|  _  |",
            "| | | |"
        ]),
        'E' => Some([
            " _____ ",
            "| ____|",
            "|  ___|",
            "| |_  |",
            "|_____|"
        ]),
        'L' => Some([
            " _    ",
            "| |   ",
            "| |   ",
            "| |   ",
            "|_|___"
        ]),
        'O' => Some([
            " _____ ",
            "|  _  |",
            "| | | |",
            "| | | |",
            "|_| |_|"
        ]),
         'W' => Some([
            "_      _",
            "| |    | |",
            "| |    | |",
            "| |    | |",
            "|_|____|_|"
        ]),
        'R' => Some([
            " ______",
            "|  ___|",
            "| |_  |",
            "|  _| |_",
            "|_|  (_)"
        ]),
        'D' => Some([
            " _____",
            "|  ___|",
            "| |_  |",
            "|  _| |_",
            "|_|  (_)"
        ]),
        ' ' => Some([
            "       ",
            "       ",
            "       ",
            "       ",
            "       "
        ]),
        _ => None, // Handle unknown characters
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: text_to_ascii <text>");
        return;
    }

    let input_text = &args[1];

    let mut ascii_lines: Vec<String> = vec!["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];

    for c in input_text.chars() {
        match char_to_ascii(c) {
            Some(ascii_char) => {
                for i in 0..5 {
                    ascii_lines[i].push_str(ascii_char[i]);
                }
            },
            None => {
                println!("Character '{}' not supported.", c);
            }
        }
    }

    for line in ascii_lines {
        println!("{}");
    }
}