use num_bigint::BigUint;
use crate::point::Point;
use finite_field::ff::FiniteField as FF;
use finite_field::helper::sqrt_root;
use num_traits::{Num, Zero};

#[derive(Debug, Clone)]
pub struct Secp256k1 {
    pub a: FF,
    pub b: FF,
    pub p: BigUint,
    pub g: Point,
    pub n: BigUint,
}

impl Secp256k1 {
    pub fn new() -> Self {
        let p = BigUint::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from(7_u32), p.clone());

        let x = BigUint::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        let y = BigUint::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();
        let x = FF::new(x, p.clone());
        let y = FF::new(y, p.clone());
        let g = Point::new(Some(x), Some(y), a.clone(), b.clone());

        let n = BigUint::from_str_radix("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();
        Self {a, b, p, g, n}
    }

    pub fn g(&self) -> &Point {
        &self.g
    }

    pub fn n(&self) -> &BigUint {
        &self.n
    }

    pub fn point(&self, x: BigUint, y: BigUint) -> Point {
        Point::new(Some(FF::new(x, self.p.clone())), Some(FF::new(y, self.p.clone())), self.a.clone(), self.b.clone())
    }

    pub fn lift_x(&self, x: BigUint) -> Point {
        let y = x.clone().pow(3) + self.a.clone().num * x.clone() + self.b.clone().num;
        let y = sqrt_root(y, self.p.clone());

        Point::new(Some(FF::new(x, self.p.clone())), 
                    Some(FF::new(y, self.p.clone())), 
                    self.a.clone(), 
                    self.b.clone())

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_secp256k1() {
        let secp256k1 = Secp256k1::new();
        let x = BigUint::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        let y = BigUint::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();
        let g = secp256k1.point(x, y);
        assert_eq!(secp256k1.g(), &g);
        assert_eq!(secp256k1.g.scalar_mul(100) + g.clone(), g.clone().scalar_mul(101));
    }

    #[test]
    pub fn test_lift_x() {
        let secp256k1 = Secp256k1::new();
        let x = BigUint::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        assert_eq!(secp256k1.lift_x(x), secp256k1.g);
        
    }
}