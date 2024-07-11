// use num_traits::Num;
// use sha2::{Sha256,Digest};
// use crate::secp256k1::Secp256k1;
// use num_bigint::{BigUint, RandomBits};
// use crate::point::Point;
// use std::str::FromStr;

// pub struct ecdsa {
//     pub e: Secp256k1,
//     pub d: BigUint,
//     pub_key: Point,
// }

// impl ecdsa {
//     pub fn new(d: BigUint) -> Self {
//         let e = Secp256k1::new();
//         let d = BigUint::from(1_u32);
//         let pub_key = e.g().clone().scalar_mul(d.clone());
//         Self {e, d, pub_key}
//     }

//     pub fn hash(&self, m: &str) -> BigUint {
//         let mut hasher = Sha256::new();
//         let m = m.as_bytes();
//         hasher.update(m);
//         let m = hasher.finalize();
//         // BigUint::from_bytes_be(m)
//         BigUint::from_str_radix(m, 16).unwrap()
    
//     }

// }

