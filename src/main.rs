use std::io::{self, Write};

mod ch3;
mod ch8;
mod common;

fn main() {
    println!("The Rust Programming Language Book - Exercises");
    loop {
        let mut option = String::new();
        println!("(1) Convert temperatures between Celsius and Fahrenheit");
        println!("(2) Generate nth Fibonacci number");
        println!("(3) Print the lyrics to the Christmas carol \"The Twelve Days of Christmas\"");
        println!("(4) Get median/mode for array of numbers");
        println!("(5) Convert string to pig latin");
        println!("(6) Administer company directory");
        println!("Any other number to exit\n");
        print!("Enter an option: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        let num = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        match num {
            1 => ch3::convert_temperature(),
            2 => ch3::nth_fibonacci(),
            3 => ch3::twelve_days_of_christmas(),
            4 => ch8::median_mode(),
            _ => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
