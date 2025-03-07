use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    
    let file_path = Path::new(filename);
    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    let mut contents = String::new();

    for line in reader.lines() {
        match line {
            Ok(content) => contents.push_str(&content),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
        contents.push('\n');
    }

    let binary_representation = hex_to_binary(&contents);
    let square_representation = binary_to_square(&binary_representation);
    
    println!("{}", contents);
    println!("{}", binary_representation);
    println!("{}", square_representation);
    
    Ok(())
}

fn hex_to_binary(hex_string: &str) -> String {
    let mut binary_string = String::new();

    for c in hex_string.chars() {
        if c == '\n' {
            binary_string.push('\n');
            continue;
        }

        if c.is_digit(16) {
            let bin_str = format!("{:04b}", c.to_digit(16).unwrap());
            binary_string.push_str(&bin_str);
        }
    }

    binary_string
}

fn binary_to_square(bin_str: &str) -> String {
    let mut square_string = String::new();

    for c in bin_str.chars() {
        if c == '\n' {
            square_string.push('\n');
            continue;
        }

        if c == '1' {
            square_string.push('█');
        } else {
            square_string.push(' ');
        }
    }

    square_string
}