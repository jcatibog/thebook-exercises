use num::BigUint;
use ordinal::Ordinal;
use std::{
    io::{self, Write},
    mem::replace,
};

fn main() {
    println!("The Rust Programming Language Book - Exercises");
    loop {
        let mut option = String::new();
        println!("(1) Convert temperatures between Celsius and Fahrenheit");
        println!("(2) Generate nth Fibonacci number");
        println!("(3) Print the lyrics to the Christmas carol \"The Twelve Days of Christmas\"");
        println!("Any other number to exit");
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
            1 => convert_temperature(),
            2 => nth_fibonacci(),
            3 => twelve_days_of_christmas(),
            _ => {
                println!("Goodbye!");
                break;
            }
        }
    }
}

fn convert_temperature() {
    print_separator();
    let mut temperature = String::new();
    print!("Enter a temperature: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let num: i32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let num_celsius = (num as f64 - 32.0) * (5.0 / 9.0);
    let num_fahrenheit = num as f64 * (9.0 / 5.0) + 32.0;
    println!("Celsius: {}, Fahrenheit: {}", num, num_fahrenheit);
    println!("Fahrenheit: {}, Celsius: {}", num, num_celsius);
    print_separator();
}

fn nth_fibonacci() {
    print_separator();

    let mut number = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let num: i64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };
    do_fibonacci(num);

    print_separator();
}

fn do_fibonacci(n: i64) {
    let mut f0: BigUint = num::zero();
    let mut f1: BigUint = num::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    println!("The {n}th Fibonacci number is: {f0}")
}

fn twelve_days_of_christmas() {
    print_separator();

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese-a-laying",
        "Seven swans-a-swimming",
        "Eight maids-a-milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve lords-a-leaping",
    ];

    for i in 1..13 {
        println!(
            "On the {} day of Christmas, my true love gave to me",
            Ordinal(i)
        );
        for day in (0..i).rev() {
            println!("{}", gifts[day]);
        }
        println!("");
    }

    print_separator();
}

fn print_separator() {
    println!("\n------------------------------\n");
}
