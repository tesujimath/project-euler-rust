use super::primes::factors;

pub fn run() {
    let f = factors(600851475143u64);
    println!("p3: {}", f.last().unwrap());
}
