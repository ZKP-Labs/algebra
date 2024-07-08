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
            panic!(
                "Num {} not in field range 0 to {}",
                num,
                prime - BigUint::one()
            );
        }
        Self {num, prime}
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
                num: num.try_into().unwrap(),
                prime: self.prime.clone(),
            }
            
        } else {
            Self {
                num: num.try_into().unwrap(),
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
                num: new_num.try_into().unwrap(),
                prime: self.prime.clone(),
            }
        } else {
            Self{
                num: result.try_into().unwrap(),
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
        // let num = self.modulo(&(self.num.clone() * other_inv.num.clone()));

        self * other_inv
        // Self {
        //     num,
        //     prime: self.prime.clone()
        // }
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

    #[test]
    fn test_addition() {
        let prime = 65537.to_biguint().unwrap();
        let a = FiniteField::new(56638.to_biguint().unwrap(), prime.clone());
        let b = FiniteField::new(15431.to_biguint().unwrap(), prime.clone());
        let result = a + b;
        let expected = FiniteField::new(6532.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let prime = 65537.to_biguint().unwrap();
        let a = FiniteField::new(2.to_biguint().unwrap(), prime.clone());
        let b = FiniteField::new(3.to_biguint().unwrap(), prime.clone());
        let result = a - b;
        let expected = FiniteField::new(65536.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multilplication() {
        let prime = 65537.to_biguint().unwrap();
        let a = FiniteField::new(23030.to_biguint().unwrap(), prime.clone());
        let b = FiniteField::new(35563.to_biguint().unwrap(), prime.clone());
        let result = a*b;
        let expected = FiniteField::new(1.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pow() {
        let prime = 65537.to_biguint().unwrap();
        let base = FiniteField::new(2542.to_biguint().unwrap(), prime.clone());
        let exp: u32 = 13053;
        let rs = base.pow(exp);
        let expected = FiniteField::new(64259.to_biguint().unwrap(), prime.clone());
        assert_eq!(rs, expected);
    }

    #[test]
    fn test_inverse() {
        let prime = 65537.to_biguint().unwrap();
        let a = FiniteField::new(40952.to_biguint().unwrap(), prime.clone());
        let result = a.inverse();
        let expected = FiniteField::new(9498.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_division() {
        let prime = 65537.to_biguint().unwrap();
        let a = FiniteField::new(40952.to_biguint().unwrap(), prime.clone());
        let b = FiniteField::new(40286.to_biguint().unwrap(), prime.clone());
        let result = a / b;
        let expected = FiniteField::new(41712.to_biguint().unwrap(), prime);
        assert_eq!(result, expected);
    }  

}