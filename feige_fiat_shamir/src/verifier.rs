use num_bigint::BigUint;
use rand::Rng;

pub struct Verifier {
    n: BigUint,
    v: Vec<BigUint>,
}

impl Verifier {
    pub fn new(n: BigUint, v: Vec<BigUint>) -> Self {
        Verifier { n, v }
    }

    pub fn challenge(&self) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let a: Vec<u32> = self.v.iter().map(|_| rng.gen_range(0..2)).collect();
        a
    }

    pub fn verify(&self, x: &BigUint, y: &BigUint, a: &[u32]) -> bool {
        if x >= &self.n || x <= &BigUint::from(0_u32) {
            panic!("Invalid x");
        }

        let mut rs = BigUint::from(1_u32);
        for (i, ai) in a.iter().enumerate() {
            match ai {
                0 => (),
                1 => {
                    rs *= self.v[i].modpow(&BigUint::from(*ai), &self.n);
                    rs %= &self.n;
                }
                _ => panic!("Invalid challenge"),
            }
        }

        if y.modpow(&BigUint::from(2_u32), &self.n) == &rs * x % &self.n
            || y.modpow(&BigUint::from(2_u32), &self.n) == &rs * (&self.n - x) % &self.n
        {
            return true;
        }

        false
    }
}
