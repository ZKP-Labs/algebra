use crate::helper::get_deg_of_var;
use ark_ff::Field;
use ark_poly::{univariate::DensePolynomial, DenseMVPolynomial, Polynomial};
use rand::Rng;
pub struct Verifier<F: Field, P: DenseMVPolynomial<F>> {
    pub c_1: F,
    pub g: P,
    pub g_part: Vec<DensePolynomial<F>>,
    pub rs: Vec<F>,
    pub num_vars: usize,
}

impl<F: Field, P: DenseMVPolynomial<F>> Verifier<F, P> {
    pub fn new(g: &P) -> Self {
        Self {
            g: g.clone(),
            c_1: F::zero(),
            g_part: vec![],
            rs: vec![],
            num_vars: g.num_vars(),
        }
    }

    /// generate random r
    pub fn random_r(&self) -> F {
        let mut rng = rand::thread_rng();

        let r: u8 = rng.gen();
        F::from(r)
    }

    /// evaluate s_i at r which s_i is a univariate polynomial produced by prover
    pub fn evaluate_si_at_r(&self, s_i: &DensePolynomial<F>, r: F) -> F {
        s_i.evaluate(&r)
    }

    /// check if s_j_1(r) = s_j(0) + s_j(1)
    pub fn check_round(&self, s_j: &DensePolynomial<F>, s_j_1_at_r: F, round: usize) -> bool {
        let s_j_0 = s_j.evaluate(&F::zero());
        let s_j_1 = s_j.evaluate(&F::one());

        if s_j_0 + s_j_1 != s_j_1_at_r {
            return false;
        }

        // check if deg(g_j) = deg(g(r0, r1, ..., X_j, ..., b_l))
        let deg_g = get_deg_of_var(&self.g, round);
        let deg_g_j = s_j.degree();

        if deg_g_j > deg_g {
            return false;
        }

        true
    }
}
