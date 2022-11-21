use std::io::{self, Write};

use crate::common;

pub fn median_mode() {
    common::print_separator();

    print!("Enter a series of numbers: ");
    io::stdout().flush().unwrap();

    common::print_separator();
}
