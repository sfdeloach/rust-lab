// read whitespace-separated integers from stdin, print min, max, sum, mean
// (as f64), and a line per value saying odd/even. Use if as an expression at
// least once, shadowing at least once, and a labeled break to stop at the
// first negative number.

use std::io::{self, Write};
fn main() {
    let mut user_input = String::new();
    let mut integers: Vec<String> = Vec::new();
    let mut index = 0;
    let mut min = 0;
    let mut max = 0;
    let mut sum = 0;

    print!("Enter integers: ");
    io::stdout().flush().expect("Failed to flush stdout.");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    'input_processing: for char in user_input.chars() {
        if char.is_numeric() {
            if integers.get(index) == None {
                integers.push(String::from(""));
            }
            integers[index].push(char);
        } else if char.is_whitespace() && integers.get(index) != None {
            let value: u32 = integers[index].parse().expect("Failed to parse.");

            // a line per value with its parity, using if as an expression
            println!("{}: {}", value, if value % 2 == 0 { "even" } else { "odd" });

            if index == 0 {
                min = value;
                max = value;
            } else {
                if value < min {
                    min = value;
                }

                if value > max {
                    max = value;
                }
            }

            sum += value;

            index += 1;
        } else if char == '-' {
            break 'input_processing;
        }
    }

    println!("----------");
    println!("min = {min}");
    println!("max = {max}");
    println!("sum = {sum}");
    println!("mean = {}", sum as f64 / index as f64);
}
