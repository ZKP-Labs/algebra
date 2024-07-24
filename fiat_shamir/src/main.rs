use num_bigint::BigUint;
use num_primes::Generator;
use num_traits::Num;
use rand::Rng;

pub struct Prover {
    n: BigUint,
    s: Vec<BigUint>,
    v: Vec<BigUint>,
}

pub struct Verifier {
    n: BigUint,
    v: Vec<BigUint>,
}

impl Prover {
    fn new(p: BigUint, q: BigUint, s: Vec<BigUint>) -> Self {
        let n = &p * &q;
        let mut v: Vec<BigUint> = Vec::new();
        for si in s.iter() {
            let vi = si.modpow(&BigUint::from(2_u32), &n);
            v.push(vi);
        }
        Prover { n, s, v }
    }

    fn commit(&self) -> (BigUint, BigUint) {
        let mut rng = rand::thread_rng();
        let r: BigUint = rng.sample(num_bigint::RandomBits::new(256));
        let x = r.modpow(&BigUint::from(2_u32), &self.n);
        (r, x)
    }

    fn respond(&self, r: &BigUint, a: &[u32]) -> BigUint {
        let mut rs = r.clone();
        for (i, ai) in a.iter().enumerate() {
            rs *= self.s[i].modpow(&BigUint::from(*ai), &self.n);
        }
        rs
    }
}

impl Verifier {
    fn new(n: BigUint, v: Vec<BigUint>) -> Self {
        Verifier { n, v }
    }

    fn challenge(&self) -> Vec<u32> {
        let mut a: Vec<u32> = Vec::new();
        let mut rng = rand::thread_rng();
        let k = self.v.len();
        for _ in 0..k {
            a.push(rng.gen_range(0..2));
        }
        a
    }

    fn verify(&self, x: &BigUint, y: &BigUint, a: &[u32]) -> bool {
        let mut rs = x.clone();
        for (i, ai) in a.iter().enumerate() {
            rs *= self.v[i].modpow(&BigUint::from(*ai), &self.n);
            rs %= &self.n;
        }

        if y.modpow(&BigUint::from(2_u32), &self.n) == rs {
            return true;
        }
        false
    }
}

pub fn identification_scheme(k: usize) -> bool {
    // setup
    let p = get_prime();
    let q = get_prime();

    let mut s: Vec<BigUint> = Vec::new();
    for _ in 0..k {
        let si = Generator::new_prime(256).to_string();
        let si = BigUint::from_str_radix(&si, 10).expect("Failed to parse prime s");
        s.push(si);
    }

    let prover = Prover::new(p, q, s);
    let verifier = Verifier::new(prover.n.clone(), prover.v.clone());

    // commit
    let (r, x) = prover.commit();
    println!("Commitment: x = {}", x);

    // challenge
    let a = verifier.challenge();
    println!("Challenge: e = {:?}", a);

    // response
    let y = prover.respond(&r, &a);
    println!("Response: y = {}", y);

    // verification
    let rs = verifier.verify(&x, &y, &a);
    println!("Verification result: {}", rs);

    rs
}

pub fn get_prime() -> BigUint {
    let p = Generator::new_prime(512).to_string();
    BigUint::from_str_radix(&p, 10).expect("Failed to parse prime p")
}

fn main() {
    let rs = identification_scheme(20);
    assert!(rs);
}
