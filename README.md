# algebra

The Algebra Learning Project is an educational initiative aimed at providing foundational implementations of finite fields and elliptic curves. These mathematical structures are essential in various areas of modern cryptography and computational algebra.

## Objectives

- **Finite Fields**: Implement basic arithmetic operations and modular arithmetic for finite fields.
- **Elliptic Curves**: Develop point addition, point doubling, and scalar multiplication for elliptic curves over finite fields.
- **Learning Focus**: Emphasize understanding and practical application of algebraic concepts in cryptography.

## Finite Field

In this project, I focus on implementing a prime field and the arithmetic operations within it. This includes defining a prime field structure and supporting various arithmetic operations such as addition, subtraction, multiplication, and division, all within the context of a prime modulus.

### Usage

Clone `finite_field` and config `Cargo.toml`

```jsx
[dependencies]
finite_field = { path = "../finite_field" }
```

### Prime Field Feature

The Prime Field feature in this project allows for exploration and manipulation of elements within a prime field. Below are examples and explanations of how to use the various functionalities provided:

### Example Usage

Path: `finite_field/examples/test_prime_field`

```rust
use finite_field::ff::FiniteField;
use finite_field::prime_field::PrimeField;
use num_bigint::BigUint;

fn main() {
    // Creating a Prime Field Element
    let prime = BigUint::from(17u32);
    let num = BigUint::from(20u32);
    let field_element = PrimeField::new(num, prime.clone());
    println!("Field element: {}", field_element);

    // Performing Arithmetic Operations
    let a = PrimeField::new(BigUint::from(5u32), prime.clone());
    let b = PrimeField::new(BigUint::from(3u32), prime.clone());

    println!("Sum: {}", a.clone() + &b);
    println!("Subtract: {}", a.clone() - &b);
    println!("Product: {}", a.clone() * &b);
    println!("Quotient: {}", a.clone() / &b);

    // Modular Inversion
    println!("Inverse: {}", a.inverse());

    // ower Computations
    println!("Power: {}", a.pow(3));

    // Zero and Order of the Field
    println!("Zero element: {}", PrimeField::zero(prime));
    println!("Field order: {}", a.order());
}
```

You can also run this example with `cargo run --example test_prime_field`

## Elliptic Curve and ECDSA

In this project, I focus on implementing the secp256k1 elliptic curve. The implementation leverages my `finite_field` project to handle elements over finite fields with the prime modulus being the secp256k1 prime. This setup not only supports the secp256k1 curve but also allows for the extension to other elliptic curves based on the same foundational point structure.

To use `elliptic_curve` package, you need to clone it and config `Cargo.toml`

```rust
[dependencies]
elliptic_curve = { path = "../elliptic_curve" }
```

### **Example Usage** secp256k1

Path: `elliptic_curve/examples/test_secp256k1`

```rust
use elliptic_curve::secp256k1::*;
use num_bigint::BigUint;
use num_traits::Num;

fn main() {
    //Initialize the Curve:
    let secp256k1 = Secp256k1::new();
    println!("Secp256k1 Curve Initialized");

    //Get the Generator Point:
    println!("Generator Point:");
    let g = secp256k1.g();
    println!("Generator Point: {:?}", g);

    //Create a Custom Point:
    print!("Custom Point: ");
    let x = BigUint::from_str_radix(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();
    let y = BigUint::from_str_radix(
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .unwrap();
    let point = secp256k1.point(x, y);
    println!("Custom Point: {:?}", point);

    //Lift an X Coordinate to a Point:
    println!("Lifted Point:");
    let secp256k1 = Secp256k1::new();
    let x = BigUint::from_str_radix(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();
    let point = secp256k1.lift_x(&x);
    println!("Lifted Point: {:?}", point);
}

```

You can also run that yourself with `cargo run --example test_secp256k1`

### ECDSA

The `Ecdsa` struct provides functionality for creating an ECDSA instance, signing messages, and verifying signatures using the secp256k1 elliptic curve. Below is a guide on how to use these features:

### Example Usage ECDSA

Here is a complete example demonstrating how to create an ECDSA instance, sign a message, and verify the signature:

Path: `elliptic_curve/examples/test_ecdsa`

```rust
use num_bigint::BigUint;
use rand::Rng;
use sha2::Sha256;
use rand::Rng;
use crate::Ecdsa;

fn main() {
    let mut rng = rand::thread_rng();
    let d: BigUint = rng.sample(num_bigint::RandomBits::new(256));

    // Create a new ECDSA instance with the private key d
    let ecdsa = Ecdsa::new(d);

    let message = "No.more.caffeine";

    // Hash the message
    let hash_value = ecdsa.hash(message);
    println!("Hash: {}", hash_value);

    // Sign the message
    let (r, s) = ecdsa.sign(message);
    println!("Signature: (r: {}, s: {})", r, s);

    // Verify the signature
    let is_valid = ecdsa.verify(message, &r, &s);
    if is_valid {
        println!("Signature is valid.");
    } else {
        println!("Signature is invalid.");
    }
}

```

You can also run that yourself with `cargo run --example test_ecdsa`

### Running Tests

To ensure that your implementation is correct, you can run the tests defined in the `tests` module:

```
cargo test
```

# SUM CHECK PROTOCOL

about sum check protocol in [here](https://github.com/nguyen-xuan-quoc/algebra/tree/feature/sum_check_protocol/sum_check_protocol/README.md)
