use num_bigint::BigUint;
use num_primes::Generator;
use num_traits::Num;
pub mod prover;
pub mod verifier;
use prover::Prover;
use verifier::Verifier;

pub fn identification_scheme(k: usize) -> bool {
    // setup
    let p = get_prime(256);
    let q = get_prime(256);

    let mut s: Vec<BigUint> = Vec::with_capacity(k);
    for _ in 0..k {
        let si = get_prime(256);
        s.push(si);
    }

    let prover = Prover::new(p, q, s);
    let verifier = Verifier::new(prover.n.clone(), prover.v.clone());

    for round in 0..k {
        println!("Round: {}", round + 1);

        // commit
        println!("Prover");
        let (r, x) = prover.commit();
        println!("Commitment: x = {}", x);

        // challenge
        println!("Verifier");
        let a = verifier.challenge();
        println!("Challenge: e = {:?}", a);

        // response
        println!("Prover");
        let y = prover.respond(&r, &a);
        println!("Response: y = {}", y);

        // verification
        println!("Verifier");
        let rs = verifier.verify(&x, &y, &a);
        println!("Verification result: {}", rs);

        if !rs {
            panic!("fail at round {}", round + 1);
        }
    }

    true
}

pub fn get_prime(bit: usize) -> BigUint {
    let p = Generator::new_prime(bit).to_string();
    BigUint::from_str_radix(&p, 10).expect("Failed to parse prime p")
}

fn main() {
    let rs = identification_scheme(50);
    assert!(rs);
}
