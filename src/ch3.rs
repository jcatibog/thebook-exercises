use num::BigUint;
use ordinal::Ordinal;
use std::{
    io::{self, Write},
    mem::replace,
};

use crate::common;

pub fn convert_temperature() {
    common::print_separator();
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
    common::print_separator();
}

pub fn nth_fibonacci() {
    common::print_separator();

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

    common::print_separator();
}

pub fn do_fibonacci(n: i64) {
    let mut f0: BigUint = num::zero();
    let mut f1: BigUint = num::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    println!("The {n}th Fibonacci number is: {f0}")
}

pub fn twelve_days_of_christmas() {
    common::print_separator();

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
            if day == 0 && i > 1 {
                print!("and ");
            }
            println!("{}", gifts[day]);
        }
        println!("");
    }

    common::print_separator();
}
