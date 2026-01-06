use figlet_rs::FIGfont;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ascii_art <text>");
        return;
    }

    let text = &args[1..].join(" ");

    let font = FIGfont::standard().unwrap();
    let figure = font.convert(text);

    match figure {
        Some(rendered) => {
            println!("{}", rendered);
        }
        None => {
            eprintln!("Failed to render ASCII art.");
        }
    }
}