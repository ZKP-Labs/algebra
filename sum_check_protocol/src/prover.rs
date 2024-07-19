use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm, Term},
    univariate::DensePolynomial,
    DenseMVPolynomial, DenseUVPolynomial, Polynomial,
};
pub use rand::Rng;

pub struct Prover<F: Field, P: DenseMVPolynomial<F>> {
    pub g: P,
    pub c_1: F,
}

impl<F> Prover<F, SparsePolynomial<F, SparseTerm>>
where
    F: Field,
{
    /// caculate claimed answer of sumcheck protocol
    pub fn calculate_c_1(g: &SparsePolynomial<F, SparseTerm>) -> F {
        let v = g.num_vars();
        let mut sum = F::zero();

        for i in 0..(1 << v) {
            let point: Vec<F> = (0..v)
                .map(|d| {
                    if (i >> d) & 1 == 1 {
                        F::one()
                    } else {
                        F::zero()
                    }
                })
                .collect();

            sum += &g.evaluate(&point);
        }
        sum
    }

    /// learn from: https://github.com/punwai/sumcheck/blob/main/src/main.rs
    /// Calculate univariate variate polynomial s_i from g and rs
    pub fn calculate_s_i(
        g: &SparsePolynomial<F, SparseTerm>,
        rs: &[F],
        round: usize,
    ) -> DensePolynomial<F> {
        let mut coeffs = vec![F::zero(); g.degree() + 1];
        let v: usize = g.num_vars();

        let rest = v - round - 1;
        for i in 0..(1 << rest) {
            let mut inputs: Vec<F> = vec![];

            inputs.extend(rs);
            inputs.push(F::zero());

            let mut counter = i;
            for _ in 0..rest {
                if counter & 1 == 0 {
                    inputs.push(F::zero());
                } else {
                    inputs.push(F::one());
                }
                counter >>= 1;
            }

            for (c, t) in g.terms.clone().into_iter() {
                let mut coeff = F::one();
                let mut deg = 0;

                for (&var, pow) in t.vars().iter().zip(t.powers()) {
                    if var == round {
                        deg = pow;
                    } else {
                        coeff *= inputs[var].pow([pow as u64]);
                    }
                }

                coeffs[deg] += c * coeff;
            }
        }

        DensePolynomial::from_coefficients_vec(coeffs)
    }

    pub fn new(g: &SparsePolynomial<F, SparseTerm>) -> Option<Self> {
        Some(Self {
            g: g.clone(),
            c_1: Prover::calculate_c_1(g),
        })
    }
}
