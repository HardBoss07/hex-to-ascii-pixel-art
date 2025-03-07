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

    println!("{}", contents);
    
    Ok(())
}