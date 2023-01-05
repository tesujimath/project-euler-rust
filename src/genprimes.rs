use num::FromPrimitive;
use num::One;
use num::Zero;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::Rem;

#[derive(Debug)]
pub struct Sieve<T> {
    primes: Vec<T>,
}

impl<T> Sieve<T>
where
    T: Zero + FromPrimitive + AddAssign,
    for<'a> &'a T: Rem<Output = T> + Add<Output = T>,
{
    pub fn new() -> Sieve<T> {
        Sieve { primes: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.primes.len()
    }

    fn is_prime(&self, n: &T) -> bool {
        self.primes.iter().all(|p| !(n % p).is_zero())
    }

    pub fn generate(&mut self, n: usize) -> &T {
        assert!(n > 0);
        for _ in 0..n {
            let mut candidate = match self.primes.len() {
                0 => T::from_u8(2u8).unwrap(),
                1 => T::from_u8(3u8).unwrap(),
                _ => self.primes.last().unwrap() + &T::from_u8(2u8).unwrap(),
            };

            while !self.is_prime(&candidate) {
                candidate += T::from_u8(2u8).unwrap();
            }

            self.primes.push(candidate);
        }

        self.primes.last().unwrap()
    }

    pub fn last(&self) -> Option<&T> {
        self.primes.last()
    }
}

pub fn factors<T>(m: &T) -> Vec<T>
where
    for<'a> T: Zero + FromPrimitive + AddAssign + Clone + DivAssign<&'a T> + One + PartialEq,
    for<'a> &'a T: Rem<Output = T> + Add<Output = T>,
{
    let mut n = (*m).clone();
    let sieve = &mut Sieve::new();
    let mut f = Vec::<T>::new();

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
    use num::BigUint;
    let n = BigUint::parse_bytes(b"123456789", 10).unwrap();
    let f = factors(&n);
    let p: BigUint = f.iter().product();
    assert_eq!(n, p);
}

proptest! {
    #[test]
    fn factors_multiply_to_factorand(n in 1..9999u32) {
        let f = factors(&n);
        let p = f.iter().product();
        assert_eq!(n, p);
    }
}
