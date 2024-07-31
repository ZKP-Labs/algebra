pub mod polynomial;
pub use polynomial::*;
pub mod kzg;
pub use kzg::*;
use pairing_ce::bls12_381::Fr;
use rand::{OsRng, Rng};

pub fn kzg() -> bool {
    println!("Trusted setup");
    let kzg = Kzg::setup(20);
    println!("Setup: {:?}", kzg.gp);

    let com = kzg.commit();
    println!("Commitment: {:?}", com);

    let mut rng = OsRng::new().unwrap();
    let u = rng.gen::<Fr>();

    let (y, proof) = kzg.prove(u);
    if !kzg.verify(&com, &y, &proof, u) {
        return false;
    }

    true
}

fn main() {
    let rs = kzg();
    println!("{}", rs);
}
