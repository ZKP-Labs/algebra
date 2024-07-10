use crate::secp256k1::Secp256k1;
use num_bigint::BigUint;
use crate::point::Point;

pub struct ecdsa {
    pub e: Secp256k1,
    pub d: BigUint,
    pub_key: Point,
}

impl ecdsa {
    // pub fn new() -> Self {
    //     let e = Secp256k1::new();
    //     let d = BigUint::from(1_u32);
    //     let pub_key = e.g().clone().s;
    //     Self {e, d, pub_key}
    // }

    // pub fn sign(&self, m: BigUint) -> (BigUint, BigUint) {
    //     let n = self.e.n().clone();
    //     let k = BigUint::from(1_u32);
    //     let g = self.e.g().clone();
    //     let r = g.clone() * k.clone();
    //     let r = r.x.unwrap().num;
    //     let s = (m.clone() + r.clone() * self.d.clone()) / k.clone();
    //     (r, s)
    // }

    // pub fn verify(&self, m: BigUint, r: BigUint, s: BigUint) -> bool {
    //     let n = self.e.n().clone();
    //     let g = self.e.g().clone();
    //     let w = s.clone().modpow(&-1, &n);
    //     let u1 = m.clone() * w.clone() % n.clone();
    //     let u2 = r.clone() * w.clone() % n.clone();
    //     let x = g.clone() * u1.clone() + self.pub_key.clone() * u2.clone();
    //     let x = x.x.unwrap().num;
    //     r == x
    // }
}