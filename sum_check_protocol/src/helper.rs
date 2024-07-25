use ark_bls12_381::Fq;
use ark_ff::Field;
use ark_poly::multivariate::Term;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};
use ark_poly::DenseMVPolynomial;
use rand::Rng;

/// Get the degree of a variable in a polynomial
pub fn deg_j<F: Field, P: DenseMVPolynomial<F>>(g: &P, variable: usize) -> usize {
    let mut max = 0_usize;
    for (_c, t) in g.terms().iter() {
        for (&var, pow) in t.vars().iter().zip(t.powers()) {
            if var == variable && pow > max {
                max = pow;
            }
        }
    }
    max
}

/// Generate a random polynomial
pub fn random_poly() -> SparsePolynomial<Fq, SparseTerm> {
    const NUM_TERMS: usize = 10;
    const NUM_TUPLES_PER_TERM: usize = 10;
    const MAX_RANDOM_VALUE: usize = 10;
    const MAX_RANDOM_POWER: usize = 5;
    let mut rng = rand::thread_rng();
    let terms: Vec<(Fq, SparseTerm)> = (0..NUM_TERMS)
        .map(|_| {
            let term: Vec<(usize, usize)> = (0..NUM_TUPLES_PER_TERM)
                .map(|_| {
                    (
                        rng.gen_range(0..MAX_RANDOM_VALUE),
                        rng.gen_range(0..MAX_RANDOM_POWER),
                    )
                })
                .collect();
            (
                Fq::from(rng.gen_range(0..MAX_RANDOM_VALUE as u32)),
                SparseTerm::new(term),
            )
        })
        .collect();
    SparsePolynomial::from_coefficients_vec(NUM_TERMS, terms)
}
