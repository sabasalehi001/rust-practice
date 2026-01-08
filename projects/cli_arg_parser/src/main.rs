use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments provided.");
        return;
    }

    let mut name: Option<String> = None;
    let mut age: Option<u32> = None;
    let mut verbose: bool = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--name" => {
                if i + 1 < args.len() {
                    name = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --name requires a value");
                    return;
                }
            }
            "--age" => {
                if i + 1 < args.len() {
                    if let Ok(parsed_age) = args[i + 1].parse::<u32>() {
                        age = Some(parsed_age);
                        i += 2;
                    } else {
                        eprintln!("Error: --age requires a valid integer");
                        return;
                    }
                } else {
                    eprintln!("Error: --age requires a value");
                    return;
                }
            }
            "--verbose" => {
                verbose = true;
                i += 1;
            }
            "--help" => {
                println!("Usage: cli_arg_parser [--name <name>] [--age <age>] [--verbose]");
                return;
            }
            _ => {
                eprintln!("Error: Unknown argument {}", args[i]);
                return;
            }
        }
    }

    if verbose {
        println!("Verbose mode enabled.");
    }

    if let Some(name_val) = &name {
        println!("Name: {}", name_val);
    }

    if let Some(age_val) = age {
        println!("Age: {}", age_val);
    }
}