use sha2::{Sha256,Digest};
use crate::secp256k1::Secp256k1;
use num_bigint::{BigUint, RandomBits};
use crate::point::Point;
use std::str::FromStr;

pub struct ecdsa {
    pub e: Secp256k1,
    pub d: BigUint,
    pub_key: Point,
}

impl ecdsa {
    pub fn new(d: BigUint) -> Self {
        let e = Secp256k1::new();
        let d = BigUint::from(1_u32);
        let pub_key = e.g().clone().scalar_mul(d.clone());
        Self {e, d, pub_key}
    }

    // use sha2::{Sha256, Digest};
    // use num_bigint::BigUint;
    
    pub fn hash(&self, m: &str) -> BigUint {
        let mut hasher = Sha256::new();
        hasher.update(m.as_bytes());
        let result = hasher.finalize();
        BigUint::from_bytes_be(&result)
    }
    

}

#[cfg(test)]

mod tests {
    use super::*;
    // use num_bigint::ToBigUint;
    // use std::str::FromStr;

    #[test]
    fn test_sign() {
        let ecdsa = ecdsa::new(BigUint::from(1_u32));
        let m = "hello";
        let h = ecdsa.hash(m);

        assert_eq!(h, BigUint::from_str("20329878786436204988385760252021328656300425018755239228739303522659023427620").unwrap());
        // let (r, s) = ecdsa.sign(h.clone());
        // assert!(ecdsa.verify(h, r, s));
    }
}