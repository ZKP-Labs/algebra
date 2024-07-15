use std::ops::{Add, Div, Mul, Sub};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use crate::helper::extended_euclidean_algorithm;

#[derive(Debug, Clone)]
pub struct PrimeField {
    pub num: BigUint,
    pub prime: BigUint,
}

pub trait FiniteField:
        Sized 
        + PartialEq
        + Add<Self, Output=Self>
        + Sub<Self, Output=Self>
        + Mul<Self, Output=Self>
        + Div<Self, Output=Self>
{

    /// Create a new number in the field
    /// 
    /// If the number is greater than the prime, it will be reduced by modulo prime
    fn new(num: BigUint, prime: BigUint) -> Self;

    /// Return the order of the field
    fn order(&self) -> BigUint;

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

impl FiniteField for PrimeField {
    fn new(num: BigUint, prime: BigUint) -> Self {
        if num >= prime {
            let num = &num % prime.clone();
            Self {num, prime}
        } else {
            Self {num, prime}
        }
    }

    fn order(&self) -> BigUint {
        self.prime.clone()
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
        let result = b % self.prime.clone();
        if result < BigUint::zero() {
            result + self.prime.clone()
        } else {
            result
        }
    }

    fn inverse(&self) -> Self {
        let a = BigInt::from(self.num.clone());
        let b = BigInt::from(self.prime.clone());
        let (gcd, num, _) = extended_euclidean_algorithm(a, b);

        if gcd != BigInt::one() {
            panic!("base is not invertible for the given modulus");
        } 
        if num < BigInt::zero() {
            let num = num + BigInt::from(self.prime.clone());
            Self {
                num: num.to_biguint().expect("err"),
                prime: self.prime.clone(),
            }
            
        } else {
            Self {
                num: num.to_biguint().expect("err"),
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
        if self.prime != other.prime {
            panic!("Can not add two numbers in different Fields");
        }
        let num = self.modulo(&(self.num.clone() + other.num));
        Self {
            num,
            prime:self.prime.clone(),
        }
    }
}

impl Sub for PrimeField {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Can not sub two numer is difference Fields");
        }   

        let result = BigInt::from(self.num.clone()) - BigInt::from(other.num.clone());

        if result < BigInt::zero() {
            let new_num = result + BigInt::from(self.prime.clone());
            Self {
                num: new_num.to_biguint().expect("err"),
                prime: self.prime.clone(),
            }
        } else {
            Self{
                num: result.to_biguint().expect("err"),
                prime: self.prime.clone(),
            }
        }
    }
}

impl Mul for PrimeField {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Can not mul two numer is difference Fields");
        }   
        let num = self.modulo(&(self.num.clone() * other.num.clone()));
        Self{
            num,
            prime: self.prime.clone()
        }
    }
}

impl Div for PrimeField {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Can not div two numer is difference Fields");
        }  

        let other_inv = other.inverse();
        self * other_inv
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;
    use std::str::FromStr;

    #[test]
    fn test_big_num(){ 
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("65590").unwrap(), prime.clone());
        assert!(a.num < a.prime)
    }

    #[test]
    fn test_order() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("65590").unwrap(), prime.clone());
        let order = a.order();
        assert_eq!(order, prime);
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
        let result = a - b;
        let expected = PrimeField::new(BigUint::from_str("65536").unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multilplication() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = PrimeField::new(BigUint::from_str("23030").unwrap(), prime.clone());
        let b = PrimeField::new(BigUint::from_str("35563").unwrap(), prime.clone());
        let result = a*b;
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