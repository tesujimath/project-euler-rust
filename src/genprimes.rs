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
struct Sieve<T> {
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

    fn is_prime(&self, n: &T) -> bool {
        self.primes.iter().all(|p| !(n % p).is_zero())
    }
}

#[gat]
impl<T> LendingIterator for Sieve<T>
where
    T: Zero + FromPrimitive + AddAssign<u8>,
    for<'a> &'a T: Rem<Output = T> + Add<u8, Output = T>,
{
    type Item<'next> = &'next T;

    fn next(&mut self) -> Option<&T> {
        let mut candidate = match self.primes.len() {
            0 => T::from_i32(2).unwrap(),
            1 => T::from_i32(3).unwrap(),
            _ => self.primes.last().unwrap() + 2u8,
        };

        while !self.is_prime(&candidate) {
            candidate += 2u8;
        }

        self.primes.push(candidate);

        self.primes.last()
    }
}

pub fn factors<T>(m: &T) -> Vec<T>
where
    for<'a> T: Zero + FromPrimitive + AddAssign<u8> + Clone + DivAssign<&'a T> + One + PartialEq,
    for<'a> &'a T: Rem<Output = T> + Add<u8, Output = T>,
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
    let n = BigUint::parse_bytes(b"123456789123456789", 10).unwrap();
    let f = factors(&n);

    println!("Factorised {:?} into {} factors: {:?}", n, f.len(), f);
    assert!(!f.is_empty());
}
