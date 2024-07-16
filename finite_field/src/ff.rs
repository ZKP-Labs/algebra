use num_bigint::BigUint;
use std::ops::{Add, Div, Mul, Sub};

pub trait FiniteField:
    Sized
    + PartialEq
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
{
    /// Create a new number in the field
    ///
    /// If the number is greater than the prime, it will be reduced by modulo prime
    fn new(num: BigUint, prime: BigUint) -> Self;

    /// Return the order of the field
    fn order(&self) -> &BigUint;

    /// Create a new number in the field with value 0 in the field
    fn zero(prime: BigUint) -> Self;

    /// Return the number raised to the power of exp
    fn pow(&self, exp: u32) -> Self;

    /// Return the inverse of the number
    ///
    /// example: 3^-1 = 2 mod 5 such that 3*2 = 1 mod 5
    fn inverse(&self) -> Self;

    /// modulo operation
    ///
    /// example: -1 % 5 = 4 (not -1)
    fn modulo(&self, b: &BigUint) -> BigUint;

    /// Return the number with value 0 in the field
    fn to_zero(&self) -> Self;
}
