use crate::point::ECCPoint as Point;
use crate::point::Point as point;
use crate::point::PointData;
use finite_field::ff::FiniteField;
use finite_field::helper::sqrt_root;
use finite_field::prime_field::PrimeField as FF;
use num_bigint::BigUint;
use num_traits::{Num, Zero};

#[derive(Debug, Clone)]
pub struct Secp256k1 {
    pub a: FF,
    pub b: FF,
    pub p: BigUint,
    pub g: Point,
    pub n: BigUint,
}

impl Default for Secp256k1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Secp256k1 {
    /// Create a new secp256k1 curve
    pub fn new() -> Self {
        let p = BigUint::from_str_radix(
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
            16,
        )
        .unwrap();
        let a = FF::new(BigUint::zero(), p.clone());
        let b = FF::new(BigUint::from(7_u32), p.clone());

        let x = BigUint::from_str_radix(
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        )
        .unwrap();
        let y = BigUint::from_str_radix(
            "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16,
        )
        .unwrap();
        let x = FF::new(x, p.clone());
        let y = FF::new(y, p.clone());
        let g: PointData = (Some(x), Some(y), a.clone(), b.clone());
        let g = Point::new(&g);

        let n = BigUint::from_str_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .unwrap();
        Self { a, b, p, g, n }
    }

    /// Return the generator point
    pub fn g(&self) -> &Point {
        &self.g
    }

    /// return the n which is the order of the generator point
    pub fn n(&self) -> &BigUint {
        &self.n
    }

    /// create a new point
    pub fn point(&self, x: BigUint, y: BigUint) -> Point {
        let p: PointData = (
            Some(FF::new(x, self.p.clone())),
            Some(FF::new(y, self.p.clone())),
            self.a.clone(),
            self.b.clone(),
        );
        Point::new(&p)
    }

    /// lift x to a point which mean return a point (x, y) such that y^2 = x^3 + ax + b mod p
    pub fn lift_x(&self, x: &BigUint) -> Point {
        let y = x.pow(3) + &self.a.num * x + &self.b.num;
        let y = sqrt_root(y, self.p.clone());

        let p: PointData = (
            Some(FF::new(x.clone(), self.p.clone())),
            Some(FF::new(y, self.p.clone())),
            self.a.clone(),
            self.b.clone(),
        );
        Point::new(&p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_secp256k1() {
        let secp256k1 = Secp256k1::new();
        let x = BigUint::from_str_radix(
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        )
        .unwrap();
        let y = BigUint::from_str_radix(
            "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16,
        )
        .unwrap();
        let g = secp256k1.point(x, y);
        assert_eq!(secp256k1.g(), &g);
        assert_eq!(
            secp256k1.g.scalar_mul(BigUint::from(100_u32)) + g.clone(),
            g.clone().scalar_mul(BigUint::from(101_u32))
        );
    }

    #[test]
    pub fn test_lift_x() {
        let secp256k1 = Secp256k1::new();
        let x = &BigUint::from_str_radix(
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        )
        .unwrap();
        assert_eq!(secp256k1.lift_x(x), secp256k1.g);
    }
}
