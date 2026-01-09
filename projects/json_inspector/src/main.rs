use std::env;
use std::fs;

use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: json_inspector <path_to_json_file>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    match serde_json::from_str::<Value>(&contents) {
        Ok(_) => {
            println!("Valid JSON");
        }
        Err(err) => {
            eprintln!("Invalid JSON: {}", err);
            std::process::exit(1);
        }
    }
}