use num::BigUint;
use num::FromPrimitive;
use num::One;
use num::Zero;

#[derive(Debug)]
pub struct Sieve {
    primes: Vec<BigUint>,
}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve { primes: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.primes.len()
    }

    fn is_coprime(&self, n: &BigUint) -> bool {
        self.primes.iter().all(|p| !(n % p).is_zero())
    }

    pub fn generate(&mut self, n: usize) -> &BigUint {
        assert!(n > 0);
        for _ in 0..n {
            let mut candidate = match self.primes.len() {
                0 => BigUint::from_u8(2u8).unwrap(),
                1 => BigUint::from_u8(3u8).unwrap(),
                _ => self.primes.last().unwrap() + &BigUint::from_u8(2u8).unwrap(),
            };

            while !self.is_coprime(&candidate) {
                candidate += BigUint::from_u8(2u8).unwrap();
            }

            self.primes.push(candidate);
        }

        self.primes.last().unwrap()
    }

    pub fn last(&self) -> Option<&BigUint> {
        self.primes.last()
    }
}

pub fn factors(m: &BigUint) -> Vec<BigUint> {
    let mut n = m.clone();
    let sieve = &mut Sieve::new();
    let mut f = Vec::<BigUint>::new();

    while !n.is_one() {
        let p = sieve.generate(1);
        while (&n % p).is_zero() {
            f.push(p.clone());
            n /= p;
        }
    }

    f
}

#[test]
fn test_factors() {
    let n = BigUint::parse_bytes(b"123456789", 10).unwrap();
    let f = factors(&n);

    println!("Factorised {:?} into {} factors: {:?}", n, f.len(), f);
    assert!(!f.is_empty());
}
