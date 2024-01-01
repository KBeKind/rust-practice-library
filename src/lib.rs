//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function for read a line from stdin.
//! # Examples:
//! ```
//! use rust-proj-5::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read line".


use std::io::{BufReader, BufRead};
// THE pub mod colors; IS USED TO ADD THE colors.rs FILE TO THE LIBRARY
pub mod colors;


/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read line".
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin: std::io::Stdin = std::io::stdin();
    let mut reader: BufReader<std::io::StdinLock<'_>> = BufReader::new(stdin.lock());
    let mut line: String = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}