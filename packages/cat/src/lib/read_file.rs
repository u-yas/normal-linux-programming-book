use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = String::new();
    // Read the file line by line
    while reader.read_line(&mut line).unwrap() > 0 {
        // Print the line
        result.push_str(&line);
        // Clear the line
        line.clear();
    }
    Ok(result)
}
