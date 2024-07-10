use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

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

pub fn sqrt_root(x: BigUint, p: BigUint) -> BigUint {
    let p1 = (p.clone() + BigUint::one()) / BigUint::from(4_u32);
    x.modpow(&p1, &p)
}