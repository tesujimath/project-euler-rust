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

fn factors(m: u64) -> Vec<u64> {
    let mut n = m;
    let sieve = &mut Sieve::new();
    let mut f = Vec::<u64>::new();

    while let Some(p) = sieve.next() {
        while n % p == 0 {
            f.push(p);
            n /= p;

            if n == 1 {
                return f;
            }
        }
    }

    f
}

pub fn run() {
    let f = factors(600851475143u64);
    println!("p3: {}", f.last().unwrap());
}
