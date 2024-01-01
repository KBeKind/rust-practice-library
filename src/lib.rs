use std::io::{BufReader, BufRead};

pub fn read_stdin() -> String {
    let stdin: std::io::Stdin = std::io::stdin();
    let mut reader: BufReader<std::io::StdinLock<'_>> = BufReader::new(stdin.lock());
    let mut line: String = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}