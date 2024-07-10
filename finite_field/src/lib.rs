use std::ops::{Add, Div, Mul, Sub};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct FiniteField {
    pub num: BigUint,
    pub prime: BigUint,
}

impl FiniteField {
    pub fn new(num: BigUint, prime: BigUint) -> Self {
        if num >= prime {
            let num = &num % prime.clone();
            Self {num, prime}
        } else {
            Self {num, prime}
        }
    }

    pub fn order(&self) -> BigUint {
        self.prime.clone()
    }


    pub fn zero(prime: BigUint) -> Self {
        Self {
            num: BigUint::zero(),
            prime,
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let num = self.modulo(&self.num.pow(exp));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn modulo(&self, b: &BigUint) -> BigUint {
        let result = b % self.prime.clone();
        if result < BigUint::zero() {
            result + self.prime.clone()
        } else {
            result
        }
    }

    pub fn inverse(&self) -> Self {
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
}

impl PartialEq for FiniteField {
    fn eq(&self, other: &FiniteField) -> bool { 
        self.num == other.num && self.prime == other.prime
    }    
}

impl Eq for FiniteField {}

impl Add for FiniteField {
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


impl Sub for FiniteField {
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


impl Mul for FiniteField {
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


impl Div for FiniteField {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Can not div two numer is difference Fields");
        }  

        let other_inv = other.inverse();
        self * other_inv
    }
}


fn extended_euclidean_algorithm(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b == BigInt::zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (gcd, x1, y1) = extended_euclidean_algorithm(b.clone(), &a % &b);
        let x = y1.clone();
        let y = x1 - (&a / &b) * y1;
        (gcd, x, y)
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
        let a = FiniteField::new(BigUint::from_str("65590").unwrap(), prime.clone());
        assert!(a.num < a.prime)
    }

    #[test]
    fn test_addition() {
        let prime = BigUint::from_str("65537").unwrap(); 
        let a = FiniteField::new(BigUint::from_str("56638").unwrap(), prime.clone());
        let b = FiniteField::new(BigUint::from_str("15431").unwrap(), prime.clone());
        let result = a + b;
        let expected = FiniteField::new(6532.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = FiniteField::new(2.to_biguint().unwrap(), prime.clone());
        let b = FiniteField::new(3.to_biguint().unwrap(), prime.clone());
        let result = a - b;
        let expected = FiniteField::new(BigUint::from_str("65536").unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multilplication() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = FiniteField::new(BigUint::from_str("23030").unwrap(), prime.clone());
        let b = FiniteField::new(BigUint::from_str("35563").unwrap(), prime.clone());
        let result = a*b;
        let expected = FiniteField::new(1.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pow() {
        let prime = BigUint::from_str("65537").unwrap();
        let base = FiniteField::new(BigUint::from_str("2542").unwrap(), prime.clone());
        let exp: u32 = 13053;
        let rs = base.pow(exp);
        let expected = FiniteField::new(BigUint::from_str("64259").unwrap(), prime.clone());
        assert_eq!(rs, expected);
    }

    #[test]
    fn test_inverse() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = FiniteField::new(BigUint::from_str("40952").unwrap(), prime.clone());
        let result = a.inverse();
        let expected = FiniteField::new(BigUint::from_str("9498").unwrap(), prime);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_division() {
        let prime = BigUint::from_str("65537").unwrap();
        let a = FiniteField::new(BigUint::from_str("40952").unwrap(), prime.clone());
        let b = FiniteField::new(BigUint::from_str("40286").unwrap(), prime.clone());
        let result = a / b;
        let expected = FiniteField::new(BigUint::from_str("41712").unwrap(), prime);
        assert_eq!(result, expected);
    }  

}