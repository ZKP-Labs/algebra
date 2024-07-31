use crate::polynomial::*;
use pairing_ce::{
    bls12_381::{Fr, G1Affine, G2Affine},
    ff::Field,
    CurveAffine, CurveProjective,
};
use rand::{OsRng, Rng};

pub struct Kzg {
    pub gp: Vec<G1Affine>,
    pub tau: G2Affine,
    pub poly: Polynomial,
    pub degree: usize,
}

impl Kzg {
    fn new(gp: Vec<G1Affine>, tau: G2Affine, poly: Polynomial, degree: usize) -> Self {
        Kzg {
            gp,
            tau,
            poly,
            degree,
        }
    }

    pub fn setup(degree: usize) -> Self {
        let mut rng = OsRng::new().unwrap();
        let secret = rng.gen::<Fr>();

        let mut cur = G1Affine::one();
        let mut gp = Vec::with_capacity(degree + 1);

        for _ in 0..=degree {
            gp.push(cur);
            cur = cur.mul(secret).into_affine();
        }

        let tau = G2Affine::one().mul(secret).into_affine();
        let poly = Polynomial::rand(degree);
        Self::new(gp, tau, poly, degree)
    }

    pub fn commit(&self) -> G1Affine {
        self.poly.commit(&self.gp)
    }

    pub fn prove(&self, z: Fr) -> (Fr, G1Affine) {
        let y = self.poly.evaluate(z);
        let mut negz = z;
        negz.negate();
        let divisor = Polynomial::new(vec![negz, Fr::one()]);

        let quo = self.poly.divide(&divisor);
        let proof = quo.commit(&self.gp);

        (y, proof)
    }

    pub fn verify(&self, comm: &G1Affine, y: &Fr, proof: &G1Affine, z: Fr) -> bool {
        let g2_tau = &self.tau;
        let g2_u = G2Affine::one().mul(z).into_affine();

        let mut lhs = proof.pairing_with(g2_tau);
        lhs.mul_assign(&proof.pairing_with(&g2_u).inverse().unwrap());

        let h = G2Affine::one();
        let gy = G1Affine::one().mul(*y).into_affine();
        let mut rhs = comm.pairing_with(&h);
        rhs.mul_assign(&gy.pairing_with(&h).inverse().unwrap());

        lhs == rhs
    }
}
