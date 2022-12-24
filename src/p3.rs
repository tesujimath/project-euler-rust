#[derive(Debug)]
struct Sieve {
    primes: Vec<u64>,
}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve { primes: Vec::new() }
    }

    fn is_prime(&self, n: u64) -> bool {
        self.primes.iter().all(|p| n % p != 0)
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let mut candidate = match self.primes.last() {
            None => 2,
            Some(2) => 3,
            Some(p) => p + 2,
        };

        while !self.is_prime(candidate) {
            candidate += 2;
        }

        self.primes.push(candidate);

        Some(candidate)
    }
}

pub fn run() {
    let s = Sieve::new();

    for p in s.take(20) {
        println!("{}", p);
    }
    //println!("p3: {:?}", s);
}
