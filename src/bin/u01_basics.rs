// read whitespace-separated integers from stdin, print min, max, sum, mean
// (as f64), and a line per value saying odd/even. Use if as an expression at
// least once, shadowing at least once, and a labeled break to stop at the
// first negative number.

use std::io::{self};
fn main() {
    let mut user_input = String::new();
    let mut integers = Vec::new();
    let mut index = 0;
    let mut may_be_negative = false;

    // gather user input
    println!(
        "Enter non-negative integers separated by whitespace. A negative number stops processing."
    );

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    'input_processing: for char in user_input.chars() {
        if char.is_numeric() && !may_be_negative {
            if integers.get(index) == None {
                integers.push(String::from(""));
            }
            integers[index].push(char);
        } else if char.is_numeric() && may_be_negative {
            break 'input_processing;
        } else if char == '-' {
            may_be_negative = true;
        } else if char.is_whitespace() && integers.get(index) != None {
            may_be_negative = false;
            index += 1;
        }
    }

    // shadow integers
    let integers: Vec<u32> = integers.iter().map(|x| x.parse().unwrap()).collect();

    // find and print minimum
    let min = integers.iter().min().copied();

    match min {
        Some(value) => println!("min = {}", value),
        None => println!("There is no minimum."),
    }

    // find and print maximum
    let max = integers.iter().max().copied();

    match max {
        Some(value) => println!("max = {}", value),
        None => println!("There is no maximum."),
    }

    // calculate and print sum
    let sum: u32 = integers.iter().sum();
    println!("sum = {sum}");

    // calculate and print mean as f64
    println!("mean = {}", sum as f64 / index as f64);

    // line per value printing even/odd using if as an expression
    for integer in integers {
        println!(
            "{integer} is {}",
            if integer % 2 == 0 { "even" } else { "odd" }
        );
    }
}
