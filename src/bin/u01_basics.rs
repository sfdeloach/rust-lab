// read whitespace-separated integers from stdin, print min, max, sum, mean
// (as f64), and a line per value saying odd/even. Use if as an expression at
// least once, shadowing at least once, and a labeled break to stop at the
// first negative number.

use std::io::{self, Write};
fn main() {
    let mut user_input = String::new();
    let mut buffer: String = String::new();
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
            let integer = buffer.parse().expect("msg");
            integers.push(integer);
            buffer.clear();
        }
    }

    let mut min = integers[0];
    let mut max = integers[0];
    let mut sum = 0;

    // a line per value with its parity, using if as an expression
    for i in &integers {
        println!("{i}: {}", if i % 2 == 0 { "even" } else { "odd" });

        if *i < min {
            min = *i;
        }

        if *i > max {
            max = *i;
        }

        sum += *i;
    }

    println!("min = {min}");
    println!("max = {max}");
    println!("sum = {sum}");
    println!("mean = {}", sum as f64 / integers.len() as f64);
}
