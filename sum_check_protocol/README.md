# SUM CHECK PROTOCOL

## Overview

Input: $V$ given oracle access to a $l-variate$ polynomial g over filed $F$.

The purpose of the sum-check protocol is for prover to provide the verifier with the following sum:

$$
H = \sum_{b_1 \in \{0,1\}} \sum_{b_2 \in \{0,1\}} ...\sum_{b_l \in \{0,1\}} g(b_1, b_2, ..., b_l)
$$

## Protocol

First we need funtion to generate multilvariate polynomial:

Code to generate a random $10-variate$ polynomial

```rust
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
```

Start: $P$ sends claimed answer $C_1$. The protocol must check that:

$$
C_1 = \sum_{b_1 \in \{0,1\}} \sum_{b_2 \in \{0,1\}} ...\sum_{b_l \in \{0,1\}} g(b_1, b_2, ..., b_l) \\
$$

Here is code to compute $C_1$ which claimed to equal the value $H$:

```rust
/// caculate claimed answer of sumcheck protocol
pub fn calculate_c_1(g: &SparsePolynomial<F, SparseTerm>) -> F {
    let v = g.num_vars();
    let mut c_1 = F::zero();

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

        c_1 += &g.evaluate(&point);
    }
    c_1
}
```

### **Round 1**:

$P$ sends univariate polynomial $s_1(X_1)$ claimed to equal:

$$
H_1(X_1) = \sum_{b2 \in \{0, 1\}} ... \sum_{b_l \in \{0, 1\}} g(X_1,b_2, ..., b_l)
$$

V check that $C_1 = s_1(0) + s_1(1)$. If this check passes, it is safe for $V$ to believe that $C_1$ is the correct answer, so long as $V$ belives that $s_1 = H_1$

How to check this? Just check that $s_1$ and $H_1$ agree at a random point $r_1$. That mean, V chooses random element $r_1$ and send $r_1$ to P. V can compute $s_1(r_1)$ directly from $P’s$ first message.

> $s_1$ is a univariate polynomial of degree at most $deg_1(g)$, rejecting if not.
>
> Here, $deg_j(g)$ denotes the degree of $g(X_1,...,X_v)$ in variable $X_j$

To implement this in Rust, first define the `Prover` and `Verifier` structs:

```rust
pub struct Prover<F: Field, P: DenseMVPolynomial<F>> {
    pub g: P,
    pub c_1: F,
}

pub struct Verifier<F: Field, P: DenseMVPolynomial<F>> {
    pub c_1: F,
    pub g: P,
    pub g_part: Vec<DensePolynomial<F>>,
    pub rs: Vec<F>,
    pub num_vars: usize,
}
```

then, we $Prover$ compute $C_1$ which clamed to equal value $H$

```rust

impl<F> Prover<F, SparsePolynomial<F, SparseTerm>>
where
    F: Field,
{
    pub fn calculate_c_1(g: &SparsePolynomial<F, SparseTerm>) -> F {
            ...
    }

    pub fn new(g: &SparsePolynomial<F, SparseTerm>) -> Option<Self> {
        Some(Self {
            g: g.clone(),
            c_1: Prover::calculate_c_1(g),
        })
    }
}

let g = random_poly();
let prover = Prover::new(&g).unwrap();
let verifier = Verifier::new(&g);
```

### Round $j$ th (1 < $j$ < $l$):

$P$ send univariate polynomial $s_j(X_J)$ clamed to equal :

$$
\sum_{b_{j+1} \in \{0, 1\}} ... \sum_{b_l \in \{0, 1\}} g(r_{1},...,r_{j-1},X_j,b_{j+1}, ..., b_l)
$$

To achieve this, honest $Prover$ needs to covert Multivariate polynomials $g(b_1, b_2, …, b_l)$ to univariate polynomial $s_j(X_j)$. This process can be complex to implement in Rust. Fortunately, I have learned from https://github.com/punwai/sumcheck/blob/main/src/main.rs and made some modifications.

```rust
/// learn from: https://github.com/punwai/sumcheck/blob/main/src/main.rs
/// Calculate univariate variate polynomial s_i from g and rs
pub fn calculate_s_i(
    g: &SparsePolynomial<F, SparseTerm>,
    rs: &[F],
    round: usize,
) -> DensePolynomial<F> {
    let mut coeffs = vec![F::zero(); g.degree() + 1];
    let l: usize = g.num_vars();

    let rest = l - round - 1;
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
```

The process involves iterating over all possible combinations of input values and evaluating the polynomial at these points, collecting coefficients to form the univariate polynomial.

Then, $V$ check $s_j$ is a univariate polynomial of degree at most $deg_j(g)$, and that :

$$
s_{j-1}(r_{j-1}) = s_j(0) + x_j(1)
$$

`check_round` funtion:

```rust
/// check if s_{j-1}(r_{j-1}) = s_j(0) + s_j(1)
pub fn check_round(&self, s_j: &DensePolynomial<F>, s_j_1_at_r: F, round: usize) -> bool {
    let s_j_0 = s_j.evaluate(&F::zero());
    let s_j_1 = s_j.evaluate(&F::one());

    if s_j_0 + s_j_1 != s_j_1_at_r {
        return false;
    }

    // check if deg(g_j) = deg(g(r0, r1, ..., X_j, ..., b_l))
    let deg_g = deg_j(&self.g, round);
    let deg_g_j = s_j.degree();

    if deg_g_j > deg_g {
        return false;
    }

    true
}
```

### Round $l$ (FinalRound):

$P$ sends univariate polynomial $s_l(X_l)$ claimed to equal

$$
H_l := g(r_1, ..., r_{l-1}, X_l)
$$

$V$ checks that $s_{l-1}(r_l - 1) = s_l(0) + s_l(1)$

$V$ picks $r_l$ at random, and needs to check that $s_l(r_l) = g(r_1,…, r_l)$, rejecting if not.

If $V$ has not yet rejected, $V$ halts and accepts

By following this protocol, we ensure that the sum check protocol is executed correctly and securely.

### Example Execution of the Sum-Check Protocol

Let $g = (x_1 + 2)(x_2 + x_3) + x_1x_3$

→ The sum of $g’s$ evaluations over the Boolean hypercube is $H = 22$

**Round 1:**

- Prover send the univariate polynomial $s_1(X_1) = 6x_1 + 8$
- Verifier checks that $s_1(0) + x_1(1) = H \ (8 + 14 = 22)$. And picks random element $r_1$ then send to prover. Suppose $r_1 = 3$

**Round 2:**

- The honest prover would respond with the univariate polynomial $s_2 = 10x_2 + 8$
- Verifier checks that $s_1(r_1) = s_2(0) + s_2(1)  = s_1(3) \ (8 + 18 = 26)$ . The verifier then sends the prover $r_2$. Suppose that $r_2 = 4$.

**Round 3:**

- The honest prover would respond with the univariate polynomial $s_3 = g(r_1, r_2, X_3) = 8x_3 + 20$
- Verifier confirms that $s_2(r_2) = s_3(0) + s_3(1) \ (48 = 20 + 28)$ . The verifier then sends the prover random field element $r_3$. Suppose that $r_3 = 7$.
- Finally, the Verifier confirms that $s_3(r_3) = g(r_1, r_2, r_3)$ (76 = 76) by making one oracle query to $g$. If this is true, the sum-check protocol is valid!

### References

_Proofs, Arguments, and Zero-Knowledge - Justin Thaler_

_ZKP MOOC Lecture 4: Interactive Proofs_

_https://mtteo.dev/posts/understanding-sumcheck-protocol/understanding-sumcheck-protocol/_
