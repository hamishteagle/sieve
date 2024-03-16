fn main() {
    let input_number: i64 = 100;
    let primes = sieve(&input_number);
    println!("Found primes {:#?}", primes)
}

fn sieve(&up_to: &i64) -> Vec<i64> {
    //Starting lowest prime = 2
    let mut prime_index: usize = 0;
    // Put all values up to the target value in the initial vector
    let mut primes = Vec::from_iter(2..up_to);

    while prime_index < primes.len() {
        let mut composite = primes[prime_index];
        while composite < up_to {
            composite += primes[prime_index];
            if primes.contains(&composite) {
                let index = primes.iter().position(|x| *x == composite).unwrap();
                primes.remove(index);
            }
        }
        prime_index += 1;
    }
    {
        primes
    }
}
