use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

/// Extended Euclidean Algorithm
///
/// input: a, b
///
/// output: (gcd, x, y) such that ax + by = gcd(a, b)
pub fn extended_euclidean_algorithm(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b == BigInt::zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (gcd, x1, y1) = extended_euclidean_algorithm(b.clone(), &a % &b);
        let x = y1.clone();
        let y = x1 - (&a / &b) * y1;
        (gcd, x, y)
    }
}

/// Legendre symbol
///
/// return 1 if x is a quadratic residue modulo p
///
/// return -1 if x is a non-quadratic residue modulo p
pub fn legendre_symbol(x: BigUint, p: BigUint) -> BigInt {
    let result = x.modpow(&((p.clone() - BigUint::one()) / BigUint::from(2_u32)), &p);
    if result == BigUint::one() {
        BigInt::one()
    } else {
        BigInt::from(-1_i32)
    }
}

/// Square root modulo p
///
/// just use for p = 3 mod 4 (example: sec256k1)
pub fn sqrt_root(x: BigUint, p: BigUint) -> BigUint {
    if legendre_symbol(x.clone(), p.clone()) == BigInt::from(-1_i32) {
        panic!("No square root modulo p");
    }

    if p.clone() % BigUint::from(4_u32) == BigUint::from(1_u32) {
        panic!("this funtion only use for p = 3 mod 4");
    }

    let p1 = (p.clone() + BigUint::one()) / BigUint::from(4_u32);
    x.modpow(&p1, &p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_extended_euclidean_algorithm() {
        let a = BigInt::from(240_u32);
        let b = BigInt::from(46_u32);
        let (gcd, x, y) = extended_euclidean_algorithm(a, b);
        assert_eq!(gcd, BigInt::from(2_u32));
        assert_eq!(x, BigInt::from(-9_i32));
        assert_eq!(y, BigInt::from(47_u32));
    }

    #[test]
    pub fn test_sqrt_root_fail() {
        let x = BigUint::from(4_u32);
        // p = 13 = 1 mod 4 => cannot find square root by this function
        let p = BigUint::from(13_u32);
        let result = sqrt_root(x, p);
        assert_eq!(result, BigUint::from(3_u32));
    }

    #[test]
    pub fn test_sqrt_root() {
        let x = BigUint::from(13_u32);
        // p = 23 = 3 mod 4
        let p = BigUint::from(23_u32);
        let result = sqrt_root(x, p);
        assert_eq!(result, BigUint::from(6_u32));
    }
}
