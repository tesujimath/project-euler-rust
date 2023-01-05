use num::FromPrimitive;
use num::One;
use num::Zero;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::Rem;
use super::bits_trait::Bits;

#[derive(Debug)]
pub struct Sieve<T> {
    primes: Vec<T>,
}

impl<T> Sieve<T>
where
    T: Zero + FromPrimitive + AddAssign + Bits + Debug,
    for<'a> &'a T: Rem<Output = T> + Add<Output = T>,
{
    pub fn new() -> Sieve<T> {
        Sieve { primes: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.primes.len()
    }

    fn is_prime(&self, n: &T) -> bool {
        let n_bits = n.bits();
        for p in self.primes.iter() {
            //println!("p {:?} with {} bits, n {:?} with {} bits", p, p.bits(), n, n_bits);

            if p.bits() * 2 > n_bits + 1 {
                return true;
            }

            if (n % p).is_zero() {
                return false;
            }
        }
        true
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
    for<'a> T: Zero + FromPrimitive + AddAssign + Bits + Clone + DivAssign<&'a T> + One + PartialEq + Debug,
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

#[test]
fn test_generic_against_primitive() {
    let mut sieve_p = super::primes::Sieve::new();
    let mut sieve_g: Sieve<u64> = Sieve::new();

    for i in 1..1000 {
        let pp = sieve_p.generate(1);
        let pg = sieve_g.generate(1);

        assert_eq!(pp, *pg);
    }
}
