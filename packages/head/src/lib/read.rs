use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(file_path: &str, lines: u16) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = String::new();
    for _ in 0..lines {
        reader.read_line(&mut line)?;
        result.push_str(&line);
        line.clear();
    }
    Ok(result)
}
