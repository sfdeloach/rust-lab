use std::io::{self, Write};

fn main() {
    println!("Fahrenheit to Celsius");

    loop {
        print!("Enter a temp in deg F or 'x' to exit: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        temp = temp.trim().to_lowercase().to_string();

        if temp == "x" {
            break;
        }

        let temp: f64 = match temp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{} deg C", (temp - 32.0) * (5.0 / 9.0));
    }
}

// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
