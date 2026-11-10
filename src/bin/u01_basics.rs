// read whitespace-separated integers from stdin, print min, max, sum, mean
// (as f64), and a line per value saying odd/even. Use if as an expression at
// least once, shadowing at least once, and a labelled break to stop at the
// first negative number.

use std::io::{self, Write};

fn atoi(buffer: &Vec<char>) -> i32 {
    let mut integer = String::new();
    for c in buffer {
        integer.push(*c);
    }

    integer.parse().expect("Unable to parse {integer}")
}

fn main() {
    let mut user_input = String::new();
    let mut buffer: Vec<char> = Vec::new();
    let mut integers: Vec<i32> = Vec::new();

    print!("Enter integers: ");
    io::stdout().flush().expect("Failed to flush stdout.");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    for char in user_input.chars() {
        if char.is_numeric() {
            buffer.push(char);
        } else if char.is_whitespace() {
            let integer = atoi(&buffer);
            integers.push(integer);
            buffer.clear();
        }
    }

    // a line per value with its parity, using if as an expression
    for i in integers {
        println!("{i}: {}", if i % 2 == 0 { "even" } else { "odd" });
    }
}
