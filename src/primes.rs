use num::BigUint;
use num::FromPrimitive;
use num::One;
use num::Zero;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::Rem;
use std::ops::ShrAssign;

/// Naive Sieve of Eratosthenes for u64
#[derive(Debug)]
pub struct FixedSieve {
    /// Consecutive ordered primes, extended on demand.
    primes: Vec<u64>,
}

impl FixedSieve {
    pub fn new() -> FixedSieve {
        FixedSieve { primes: Vec::new() }
    }

    /// Returns whether `n` is coprime with respect to `self.primes`
    fn is_coprime(&self, n: u64) -> bool {
        self.primes.iter().all(|p| n % p != 0)
    }

    /// Generates next prime and returns it.
    pub fn generate(&mut self) -> u64 {
        let mut candidate = match self.primes.len() {
            0 => 2,
            1 => 3,
            _ => self.primes.last().unwrap() + 2,
        };

        while !self.is_coprime(candidate) {
            candidate += 2;
        }

        self.primes.push(candidate);

        *self.primes.last().unwrap()
    }

    /// Gets zero-based `i`th prime, generating as many intermediate primes as are required.
    pub fn get(&mut self, i: usize) -> u64 {
        while self.primes.len() <= i {
            let _ = self.generate();
        }

        *self.primes.get(i).unwrap()
    }

    /// Returns factors of `m`, generating primes as required.
    pub fn factors(&mut self, m: u64) -> Vec<u64> {
        let mut n = m;
        let mut f = Vec::<u64>::new();

        for i in 0usize.. {
            let p = self.get(i);
            while n % p == 0 {
                f.push(p);
                n /= p;
            }

            if n == 1 {
                break;
            }
        }

        f
    }
}

impl Iterator for FixedSieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(self.generate())
    }
}

/// Naive Sieve of Eratosthenes for BigUint.
#[derive(Debug)]
pub struct BigSieve {
    primes: Vec<BigUint>,
}

impl BigSieve {
    pub fn new() -> BigSieve {
        BigSieve { primes: Vec::new() }
    }

    /// Returns whether `n` is coprime with respect to `self.primes`
    fn is_coprime(&self, n: &BigUint) -> bool {
        self.primes.iter().all(|p| !(n % p).is_zero())
    }

    /// Generates next prime and returns it.
    pub fn generate(&mut self) -> &BigUint {
        let mut candidate = match self.primes.len() {
            0 => BigUint::from_u8(2u8).unwrap(),
            1 => BigUint::from_u8(3u8).unwrap(),
            _ => self.primes.last().unwrap() + &BigUint::from_u8(2u8).unwrap(),
        };

        while !self.is_coprime(&candidate) {
            candidate += BigUint::from_u8(2u8).unwrap();
        }

        self.primes.push(candidate);

        self.primes.last().unwrap()
    }

    /// Gets zero-based `i`th prime, generating as many intermediate primes as are required.
    pub fn get(&mut self, i: usize) -> &BigUint {
        while self.primes.len() <= i {
            let _ = self.generate();
        }

        self.primes.get(i).unwrap()
    }

    /// Returns factors of `m`, generating primes as required.
    pub fn factors(&mut self, m: &BigUint) -> Vec<BigUint> {
        let mut n = m.clone();
        let mut f = Vec::<BigUint>::new();

        for i in 0usize.. {
            let p = self.get(i);
            while (&n % p).is_zero() {
                f.push(p.clone());
                n /= p;
            }

            if n.is_one() {
                break;
            }
        }

        f
    }
}

#[test]
fn test_big_sieve_factors() {
    let s = &mut BigSieve::new();
    let n = BigUint::parse_bytes(b"123456789", 10).unwrap();
    let f = s.factors(&n);

    println!("Factorised {:?} into {} factors: {:?}", n, f.len(), f);
    assert!(!f.is_empty());
}

/// Naive generic Sieve of Eratosthenes returning references,
/// supporting both primitive integers and bigints.
#[derive(Debug)]
pub struct RefSieve<T> {
    primes: Vec<T>,
}

impl<T> RefSieve<T>
where
    T: Zero + FromPrimitive + AddAssign + Bits + Debug,
    for<'a> &'a T: Rem<Output = T> + Add<Output = T>,
{
    pub fn new() -> RefSieve<T> {
        RefSieve { primes: Vec::new() }
    }

    /// Returns whether `n` is coprime with respect to `self.primes`
    fn is_coprime(&self, n: &T) -> bool {
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

    /// Generates next prime and returns it.
    pub fn generate(&mut self) -> &T {
        let mut candidate = match self.primes.len() {
            0 => T::from_u8(2u8).unwrap(),
            1 => T::from_u8(3u8).unwrap(),
            _ => self.primes.last().unwrap() + &T::from_u8(2u8).unwrap(),
        };

        while !self.is_coprime(&candidate) {
            candidate += T::from_u8(2u8).unwrap();
        }

        self.primes.push(candidate);

        self.primes.last().unwrap()
    }

    /// Gets zero-based `i`th prime, generating as many intermediate primes as are required.
    pub fn get(&mut self, i: usize) -> &T {
        while self.primes.len() <= i {
            let _ = self.generate();
        }

        self.primes.get(i).unwrap()
    }

    /// Returns factors of `m`, generating primes as required.
    pub fn factors(&mut self, m: &T) -> Vec<T>
    where
        for<'a> T: Zero
            + FromPrimitive
            + AddAssign
            + Bits
            + Clone
            + DivAssign<&'a T>
            + One
            + PartialEq
            + Debug,
        for<'a> &'a T: Rem<Output = T> + Add<Output = T>,
    {
        let mut n = (*m).clone();
        let mut f = Vec::<T>::new();

        for i in 0usize.. {
            let p = self.get(i);
            while (&n % p).is_zero() {
                f.push(p.clone());
                n /= p;
            }

            if n.is_one() {
                break;
            }
        }

        f
    }
}

#[test]
fn test_RefSieve_factors() {
    use num::BigUint;
    let s = &mut RefSieve::new();
    let n = BigUint::parse_bytes(b"123456789", 10).unwrap();
    let f = s.factors(&n);
    let p: BigUint = f.iter().product();
    assert_eq!(n, p);
}

proptest! {
    #[test]
    fn test_ref_sieve_factors_multiply_to_factorand(n in 1..9999u32) {
        let s = &mut RefSieve::new();
        let f = s.factors(&n);
        let p = f.iter().product();
        assert_eq!(n, p);
    }
}

#[test]
fn test_ref_sieve_generic_against_primitive() {
    let mut sieve_p = super::primes::FixedSieve::new();
    let mut sieve_g: RefSieve<u64> = RefSieve::new();

    for i in 0..1000 {
        let pp = sieve_p.get(i);
        let pg = sieve_g.get(i);

        assert_eq!(pp, *pg);
    }
}

/// Defines a function to count bits required to represent an integer.
pub trait Bits {
    /// Returns number of bits required to represent an integer.
    fn bits(&self) -> usize;
}

impl Bits for BigUint {
    fn bits(&self) -> usize {
        self.bits() as usize
    }
}

/// Marker trait to support different Bits implementations for BigUint and primitives
trait Primitive {}
impl Primitive for i128 {}
impl Primitive for i64 {}
impl Primitive for i32 {}
impl Primitive for i16 {}
impl Primitive for i8 {}
impl Primitive for isize {}
impl Primitive for u128 {}
impl Primitive for u64 {}
impl Primitive for u32 {}
impl Primitive for u16 {}
impl Primitive for u8 {}
impl Primitive for usize {}

impl<B: ShrAssign<u8> + Copy + PartialEq + Zero + Primitive> Bits for B {
    fn bits(&self) -> usize {
        let mut n = 0_usize;
        let mut x = *self;
        while !x.is_zero() {
            x >>= 1u8;
            n += 1;
        }
        n
    }
}

#[test]
fn test_bits_biguint() {
    fn hex(digits: &[u8]) -> impl Bits {
        BigUint::parse_bytes(digits, 16).unwrap()
    }

    assert_eq!(hex(b"7").bits(), 3);
    assert_eq!(hex(b"5913").bits(), 15);
}

#[test]
fn test_bits_primitive() {
    assert_eq!(7_u8.bits(), 3);
    assert_eq!(0x5913_u16.bits(), 15);
    assert_eq!(0x39131234_u32.bits(), 30);
}
