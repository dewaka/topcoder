mod algo;
mod problems;

fn print_primes(limit: i64) {
    let s = algo::prime_sieve::Sieve::new(limit);
    // println!("Primes: {:?}", s);

    for p in s.primes {
        print!("{} ", p);
    }
    println!();
}

fn main() {
    println!("Hello, world!");
    print_primes(1000000000);
}
