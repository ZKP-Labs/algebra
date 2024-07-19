use crate::ff::FiniteField;
use crate::helper::extended_euclidean_algorithm;
use core::fmt;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct PrimeField {
    pub num: BigUint,
    pub prime: BigUint,
}

impl FiniteField for PrimeField {
    fn new(num: BigUint, prime: BigUint) -> Self {
        if num >= prime {
            let num = &num % &prime;
            Self { num, prime }
        } else {
            Self { num, prime }
        }
    }

    fn order(&self) -> &BigUint {
        &self.prime
    }

    fn zero(prime: BigUint) -> Self {
        Self {
            num: BigUint::zero(),
            prime,
        }
    }

    fn pow(&self, exp: u32) -> Self {
        let num = self.modulo(&self.num.pow(exp));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    fn modulo(&self, b: &BigUint) -> BigUint {
        let result = b % &self.prime;
        if result < BigUint::zero() {
            result + &self.prime
        } else {
            result
        }
    }

    fn inverse(&self) -> Self {
        let a = self.num.to_bigint().unwrap();
        let b = self.prime.to_bigint().unwrap();
        let (gcd, num, _) = extended_euclidean_algorithm(a, b);

        if gcd != BigInt::one() {
            panic!("base is not invertible for the given modulus");
        }
        if num < BigInt::zero() {
            let num = num + self.prime.to_bigint().unwrap();
            Self {
                num: num
                    .to_biguint()
                    .expect("Can not convert your input to BigUint"),
                prime: self.prime.clone(),
            }
        } else {
            Self {
                num: num
                    .to_biguint()
                    .expect("Can not convert your input to BigUint"),
                prime: self.prime.clone(),
            }
        }
    }

    fn to_zero(&self) -> Self {
        Self::zero(self.prime.clone())
    }
}

impl PartialEq for PrimeField {
    fn eq(&self, other: &PrimeField) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Add for PrimeField {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self + &other
    }
}

impl Sub for PrimeField {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self - &other
    }
}

impl Mul for PrimeField {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        self * &other
    }
}

impl Div for PrimeField {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self / &other
    }
}

impl fmt::Display for PrimeField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl<'a> Add<&'a Self> for PrimeField {
    type Output = Self;
    fn add(self, other: &Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different fields");
        }
        let num = self.modulo(&(self.num.clone() + other.num.clone()));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl<'a> Mul<&'a Self> for PrimeField {
    type Output = Self;
    fn mul(self, other: &Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different fields");
        }
        let num = self.modulo(&(self.num.clone() * other.num.clone()));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl<'a> Sub<&'a Self> for PrimeField {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot subtract two numbers in different fields");
        }
        let result = BigInt::from(self.num.clone()) - BigInt::from(rhs.num.clone());
        if result < BigInt::zero() {
            let new_num = result + BigInt::from(self.prime.clone());
            Self {
                num: new_num
                    .to_biguint()
                    .expect("Can not convert your input to BigUint"),
                prime: self.prime.clone(),
            }
        } else {
            Self {
                num: result
                    .to_biguint()
                    .expect("Can not convert your input to BigUint"),
                prime: self.prime.clone(),
            }
        }
    }
}

impl<'a> Div<&'a Self> for PrimeField {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot divide two numbers in different fields");
        }
        let rhs_inv = rhs.inverse();
        self * rhs_inv
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::{BigUint, ToBigUint};
    use std::str::FromStr;

    #[test]
    fn test_big_num() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("65590").unwrap(), prime.clone());
        assert!(a.num < a.prime)
    }

    #[test]
    fn test_order() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("65590").unwrap(), prime.clone());
        let order = a.order();
        assert_eq!(order, &prime);
    }

    #[test]
    fn test_zero() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("65590").unwrap(), prime.clone());
        let zero = a.to_zero();
        assert_eq!(zero.num, BigUint::zero());
        assert_eq!(zero, PrimeField::zero(prime));
    }

    #[test]
    fn test_addition() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("56638").unwrap(), prime.clone());
        let b = PrimeField::new(BigUint::from_str("15431").unwrap(), prime.clone());
        let result = a + b;
        let expected = PrimeField::new(6532.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(2.to_biguint().unwrap(), prime.clone());
        let b = PrimeField::new(3.to_biguint().unwrap(), prime.clone());
        let result = a.clone() - &b;
        let expected = PrimeField::new(BigUint::from_str("65536").unwrap(), prime.clone());
        assert_eq!(result, expected);

        let result = a - b;
        let expected = PrimeField::new(BigUint::from_str("65536").unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multilplication() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("23030").unwrap(), prime.clone());
        let b = PrimeField::new(BigUint::from_str("35563").unwrap(), prime.clone());
        let result = a * b;
        let expected = PrimeField::new(1.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pow() {
        let prime = BigUint::from_str("65537").unwrap();
        let base = PrimeField::new(BigUint::from_str("2542").unwrap(), prime.clone());
        let exp: u32 = 13053;
        let rs = base.pow(exp);
        let expected = PrimeField::new(BigUint::from_str("64259").unwrap(), prime.clone());
        assert_eq!(rs, expected);
    }

    #[test]
    fn test_inverse() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("40952").unwrap(), prime.clone());
        let result = a.inverse();
        let expected = PrimeField::new(BigUint::from_str("9498").unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_division() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("40952").unwrap(), prime.clone());
        let b = PrimeField::new(BigUint::from_str("40286").unwrap(), prime.clone());
        let result = a / b;
        let expected = PrimeField::new(BigUint::from_str("41712").unwrap(), prime);
        assert_eq!(result, expected);
    }
}
