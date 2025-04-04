use std::io::{self, stdout, Write};

use timestamp_verify::verify;

// Letting this panic because it's quick and dirty
fn get_int_from_stdin() -> i64 {
    let mut iline = String::new();
    io::stdin()
        .read_line(&mut iline)
        .expect("Couldn't read input");

    iline.trim().parse().expect("Couldn't parse input")
}

fn get_string_from_stdin() -> String {
    let mut iline = String::new();
    io::stdin()
        .read_line(&mut iline)
        .expect("Couldn't read input");
    iline
}

fn main() {
    print!("Input timestamp: ");
    stdout().flush().unwrap();

    let timestamp = get_int_from_stdin();

    print!("Input MAC: ");
    stdout().flush().unwrap();

    let signature = get_string_from_stdin();

    verify(timestamp, &signature).expect("Could not verify signature");
}
