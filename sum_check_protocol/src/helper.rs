use ark_bls12_381::Fq;
use ark_ff::Field;
use ark_poly::multivariate::Term;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};
use ark_poly::DenseMVPolynomial;
use rand::Rng;

pub fn get_variable_degree<F, P>(g: &P, variable: usize) -> usize
where
    F: Field,
    P: DenseMVPolynomial<F>,
{
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
    let mut rng = rand::thread_rng();
    let mut terms = vec![];
    for _ in 0..10 {
        let mut term = vec![];
        for _ in 0..10 {
            term.push((rng.gen_range(0..10), rng.gen_range(0..5)));
        }
        terms.push((Fq::from(rng.gen_range(0..10)), SparseTerm::new(term)));
    }
    SparsePolynomial::from_coefficients_vec(10, terms)
}
