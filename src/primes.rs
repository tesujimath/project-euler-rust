#[derive(Debug)]
pub struct Sieve {
    primes: Vec<u64>,
}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve { primes: Vec::new() }
    }

    fn is_coprime(&self, n: u64) -> bool {
        self.primes.iter().all(|p| n % p != 0)
    }

    pub fn generate(&mut self, n: usize) -> u64 {
        assert!(n > 0);
        for _ in 0..n {
            let mut candidate = match self.primes.len() {
                0 => 2,
                1 => 3,
                _ => self.primes.last().unwrap() + 2,
            };

            while !self.is_coprime(candidate) {
                candidate += 2;
            }

            self.primes.push(candidate);
        }

        *self.primes.last().unwrap()
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(self.generate(1))
    }
}

pub fn factors(m: u64) -> Vec<u64> {
    let mut n = m;
    let sieve = &mut Sieve::new();
    let mut f = Vec::<u64>::new();

    for p in sieve.by_ref() {
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
