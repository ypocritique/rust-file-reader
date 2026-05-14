use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a file path is provided
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;                                                                                                                                                                                                                                                                       
    }

    let file_path = &args[1];

    // Attempt to open the specified file
    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error);
                
            }
            _ => {
                panic!("Error opening file: {}", error);
            }
        },              
    };

    // Read the file line by line and print its contents
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => println!("Error reading line: {}", error),
        }
    }
}