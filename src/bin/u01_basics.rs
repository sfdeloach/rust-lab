// read whitespace-separated integers from stdin, print min, max, sum, mean
// (as f64), and a line per value saying odd/even. Use if as an expression at
// least once, shadowing at least once, and a labelled break to stop at the
// first negative number.

use std::io::{self, Write};

fn main() {
    print!("Enter integers: ");
    io::stdout().flush().expect("Failed to flush stdout.");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    let mut buffer: Vec<char> = Vec::new();
    let mut integers: Vec<i32> = Vec::new();

    for char in user_input.chars() {
        if char.is_numeric() {
            buffer.push(char);
        }
        if char.is_whitespace() {

        }
    }
}
