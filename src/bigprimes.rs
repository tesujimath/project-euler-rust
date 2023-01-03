use lending_iterator::gat;
use lending_iterator::prelude::*;
use lending_iterator::LendingIterator;
use num::BigUint;
use num::FromPrimitive;
use num::One;
use num::Zero;

#[derive(Debug)]
struct Sieve {
    primes: Vec<BigUint>,
}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve { primes: Vec::new() }
    }

    fn is_prime(&self, n: &BigUint) -> bool {
        self.primes.iter().all(|p| !(n % p).is_zero())
    }
}

#[gat]
impl LendingIterator for Sieve {
    type Item<'next> = &'next BigUint;

    fn next(&mut self) -> Option<&BigUint> {
        let mut candidate = match self.primes.len() {
            0 => BigUint::from_i32(2).unwrap(),
            1 => BigUint::from_i32(3).unwrap(),
            _ => self.primes.last().unwrap() + 2u8,
        };

        while !self.is_prime(&candidate) {
            candidate += 2u8;
        }

        self.primes.push(candidate);

        self.primes.last()
    }
}

pub fn factors(m: &BigUint) -> Vec<BigUint> {
    let mut n = m.clone();
    let sieve = &mut Sieve::new();
    let mut f = Vec::<BigUint>::new();

    while let Some(p) = sieve.next() {
        while (&n % p).is_zero() {
            f.push(p.clone());
            n /= p;

            if n.is_one() {
                return f;
            }
        }
    }

    f
}

#[test]
fn test_factors() {
    let n = BigUint::parse_bytes(b"123456789123456789", 10).unwrap();
    let f = factors(&n);

    println!("Factorised {:?} into {} factors: {:?}", n, f.len(), f);
    assert!(!f.is_empty());
}
