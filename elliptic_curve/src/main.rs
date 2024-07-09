use num_bigint::{BigInt, BigUint, ToBigUint};
use finite_field::FiniteField as FF ;
use std::str::FromStr;

fn main() {
    // println!("Hello, world!");
    let prime = BigUint::from_str("56638").unwrap();
    let a = FF::new(BigUint::from_str("200").unwrap(), prime.clone());
    println!("{}", a.num);
}
