use num_bigint::BigInt;
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