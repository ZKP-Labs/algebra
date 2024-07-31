use pairing_ce::{
    bls12_381::{Fr, G1Affine},
    ff::Field,
    GenericCurveAffine, GenericCurveProjective,
};
use rand::{OsRng, Rng};

pub struct Polynomial {
    pub coeffs: Vec<Fr>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<Fr>) -> Self {
        Polynomial { coeffs }
    }

    pub fn rand(degree: usize) -> Self {
        let mut rng = OsRng::new().unwrap();
        let coeffs = (0..degree).map(|_| rng.gen::<Fr>()).collect();
        Polynomial { coeffs }
    }

    pub fn evaluate(&self, x: Fr) -> Fr {
        self.coeffs
            .iter()
            .fold((Fr::zero(), Fr::one()), |(mut res, mut cur), &coeff| {
                let mut term = cur;
                term.mul_assign(&coeff);
                res.add_assign(&term);
                cur.mul_assign(&x);
                (res, cur)
            })
            .0
    }

    pub fn commit(&self, gp: &[G1Affine]) -> G1Affine {
        let mut res = <G1Affine as GenericCurveAffine>::Projective::zero();

        for (coeff, base) in self.coeffs.iter().zip(gp.iter()) {
            let term = base.mul(*coeff);
            res.add_assign(&term);
        }

        res.into_affine()
    }

    // q(x) = (p(x) - v) / (x - z)
    pub fn divide(&self, divisor: &Self) -> Self {
        let mut dividend = self.coeffs.clone();
        let mut coeffs = Vec::with_capacity(self.coeffs.len());

        let mut dividend_pos = dividend.len().wrapping_sub(1);
        let divisor_pos = divisor.coeffs.len().wrapping_sub(1);
        let mut difference = dividend_pos as isize - divisor_pos as isize;

        while difference >= 0 {
            let mut term_quotient = dividend[dividend_pos];
            term_quotient.mul_assign(
                &divisor.coeffs[divisor_pos]
                    .inverse()
                    .expect("Division by zero encountered"),
            );
            coeffs.push(term_quotient);

            for i in (0..=divisor_pos).rev() {
                let idx = (difference as usize).wrapping_add(i);
                let mut term = divisor.coeffs[i];
                term.mul_assign(&term_quotient);
                dividend[idx].sub_assign(&term);
            }

            if dividend_pos == 0 {
                break;
            }
            dividend_pos -= 1;
            difference -= 1;
        }

        coeffs.reverse();
        Polynomial { coeffs }
    }
}
