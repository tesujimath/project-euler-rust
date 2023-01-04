use lending_iterator::gat;
use lending_iterator::prelude::*;
use lending_iterator::LendingIterator;
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
    T: Zero,
    for<'a> &'a T: Rem<Output = T>,
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
}

#[gat]
impl<T> LendingIterator for Sieve<T>
where
    T: Zero + FromPrimitive + AddAssign,
    for<'a> &'a T: Rem<Output = T> + Add<Output = T>,
{
    type Item<'next> = &'next T;

    fn next(&mut self) -> Option<&T> {
        let mut candidate = match self.primes.len() {
            0 => T::from_u8(2u8).unwrap(),
            1 => T::from_u8(3u8).unwrap(),
            _ => self.primes.last().unwrap() + &T::from_u8(2u8).unwrap(),
        };

        while !self.is_prime(&candidate) {
            candidate += T::from_u8(2u8).unwrap();
        }

        self.primes.push(candidate);

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
