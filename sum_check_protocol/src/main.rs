use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm},
    DenseMVPolynomial, Polynomial,
};
pub mod helper;
pub mod prover;
pub mod verifier;
pub use helper::*;
pub use prover::*;
pub use verifier::*;

/// demo of sumcheck protocol
pub fn sumcheck_protocol<F: Field>(
    p: &Prover<F, SparsePolynomial<F, SparseTerm>>,
    mut v: Verifier<F, SparsePolynomial<F, SparseTerm>>,
) -> bool {
    let num_rounds = p.g.num_vars();

    //first round
    let c_1 = p.c_1;
    let s_1 = Prover::calculate_s_i(&p.g, &v.rs, 0);
    v.g_part.push(s_1.clone());

    if !v.check_round(&s_1, c_1, 0) {
        panic!("fail at round 0");
    }

    // first r
    let mut r_i = v.random_r();
    v.rs.push(r_i);

    //other rounds
    for round in 1..num_rounds {
        // calculate s_{j-1} at r
        let s_j_1_at_r = v.evaluate_si_at_r(&v.g_part[round - 1], v.rs[round - 1]);

        // prover calculate s_j
        let s_j = Prover::calculate_s_i(&p.g, &v.rs, round);
        // println!("s_j: {:?}", s_j);
        v.g_part.push(s_j.clone());

        // verifier check that s_{j-1}(r-1) = s_j(0) + s_j(1)
        if !v.check_round(&s_j, s_j_1_at_r, round) {
            panic!("Fail ar round {}", round);
        }

        // random r for next round
        r_i = v.random_r();
        v.rs.push(r_i);
    }

    true
}

fn main() {
    // let g = random_poly();
    let g = random_poly();
    // println!("poly g: {:?}", g);
    let prover = Prover::new(&g).unwrap();
    let verifier = Verifier::new(&g);
    assert!(sumcheck_protocol(&prover, verifier));
    println!("sumcheck protocol pass");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prover() {
        let g = random_poly();
        let prover = Prover::new(&g).unwrap();
        assert_eq!(prover.g, g);
    }

    #[test]
    fn test_verifier() {
        let g = random_poly();
        let verifier = Verifier::new(&g);
        assert_eq!(verifier.g, g);
    }

    #[test]
    fn test_sum_check_protocol() {
        let g = random_poly();
        let prover = Prover::new(&g).unwrap();
        let verifier = Verifier::new(&g);
        assert!(sumcheck_protocol(&prover, verifier));
    }
}
