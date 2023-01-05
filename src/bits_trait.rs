use num::BigUint;
use num::Zero;
use std::ops::ShrAssign;

pub trait Bits {
    fn bits(&self) -> usize;
}

impl Bits for BigUint {
    fn bits(&self) -> usize {
        self.bits() as usize
    }
}

// a marker trait so I can have different implementations for BigUint and primitives
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
