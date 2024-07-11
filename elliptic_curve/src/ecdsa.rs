use sha2::{Sha256,Digest};
use crate::secp256k1::Secp256k1;
use num_bigint::{BigUint, RandomBits};
use crate::point::Point;
use std::str::FromStr;
use rand::Rng;

pub struct Ecdsa {
    pub e: Secp256k1,
    pub d: BigUint,
    pub_key: Point,
}

impl Ecdsa {
    pub fn new(d: BigUint) -> Self {
        let e = Secp256k1::new();
        let pub_key = e.g().clone().scalar_mul(d.clone());
        Self {e, d, pub_key}
    }

    pub fn hash(&self, m: &str) -> BigUint {
        let mut hasher = Sha256::new();
        hasher.update(m.as_bytes());
        let result = hasher.finalize();
        BigUint::from_bytes_be(&result)
    }

    pub fn sign(&self, m: &str) -> (BigUint, BigUint) {
        let mut rng = rand::thread_rng();
        let z = self.hash(m);
        let k: BigUint = rng.sample(RandomBits::new(256));
        let r = self.e.g().clone().scalar_mul(k.clone()).x().num;
        let k_inv = k.modpow(&(self.e.n() - BigUint::from(2_u32)), self.e.n());
        let s = (z.clone() + r.clone()*self.d.clone()) * k_inv.clone() % self.e.n();
        (r, s)
    }

    pub fn verify(&self, m: &str, r: BigUint, s: BigUint) -> bool {
        let z = self.hash(m);
        let s_inv = s.modpow(&(self.e.n() - BigUint::from(2_u32)), self.e.n());
        let u1 = z.clone() * s_inv.clone() % self.e.n();
        let u2 = r.clone() * s_inv.clone() % self.e.n();
        let p = self.e.g().clone().scalar_mul(u1.clone()) + self.pub_key.clone().scalar_mul(u2.clone());
        r == p.x().num % self.e.n()
    }


}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        let mut rng = rand::thread_rng();
        let d: BigUint = rng.sample(RandomBits::new(256));
        let ecdsa = Ecdsa::new(d.clone());
        let m = "hello";
        let h = ecdsa.hash(m);
        assert_eq!(h, BigUint::from_str("20329878786436204988385760252021328656300425018755239228739303522659023427620").unwrap());
    }

    fn test_sign_verify() {
        let mut rng = rand::thread_rng();
        let d: BigUint = rng.sample(RandomBits::new(256));
        let ecdsa = Ecdsa::new(d.clone());
        let m = "no.more.caffeine";
        let (r, s) = ecdsa.sign(m);
        assert!(ecdsa.verify(m, r, s));
    }
}