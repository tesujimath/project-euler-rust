use super::primes::FixedSieve;

pub fn run() {
    let mut s = FixedSieve::new();
    let f = s.factors(600851475143u64);
    println!("p3: {}", f.last().unwrap());
}
