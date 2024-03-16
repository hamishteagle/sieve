mod sieve;
use std::io;

fn main() {
    let mut input_number = String::new();

    println!("Calculate prime values up to ..");
    io::stdin()
        .read_line(&mut input_number)
        .expect("Please provide a valid number to calculate primes up to");

    let input_number: i64 = input_number.trim().parse().expect("Must be a number!");
    let primes = sieve::sieve(&input_number);
    println!("Found primes {:#?}", primes)
}
