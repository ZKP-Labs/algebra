use crate::secp256k1::Secp256k1;
use num_bigint::BigUint;

pub struct ecdsa {
    pub e: Secp256k1,
    pub d: BigUint,
    pub k: BigUint,
}