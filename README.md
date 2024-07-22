# algebra

The Algebra Learning Project is an educational initiative aimed at providing foundational implementations of finite fields and elliptic curves. These mathematical structures are essential in various areas of modern cryptography and computational algebra.

### Objectives

- **Finite Fields**: Implement basic arithmetic operations and modular arithmetic for finite fields.
- **Elliptic Curves**: Develop point addition, point doubling, and scalar multiplication for elliptic curves over finite fields.
- **Learning Focus**: Emphasize understanding and practical application of algebraic concepts in cryptography.

## Finite Field

In this project, I focus on implementing a prime field and the arithmetic operations within it. This includes defining a prime field structure and supporting various arithmetic operations such as addition, subtraction, multiplication, and division, all within the context of a prime modulus.

### Usage

Clone `finite_field` and config `Cargo.toml`

```rust
[dependencies]
finite_field = { path = "../finite_field" }
```

### Usage of the Prime Field Feature

The Prime Field feature in this project allows for exploration and manipulation of elements within a prime field. Below are examples and explanations of how to use the various functionalities provided:

### Creating a Prime Field Element

To create a new element within a prime field, you initialize it with a number and a prime. If the number exceeds the prime, it is reduced modulo the prime.

```rust
use finite_field::ff::FiniteField;
use finite_field::prime_field::PrimeField;

use num_bigint::BigUint;
use finite_field::PrimeField;

let prime = BigUint::from(17u32);
let num = BigUint::from(20u32);
let field_element = PrimeField::new(num, prime); // Creates an element equivalent to 20 % 17 = 3
```

### Performing Arithmetic Operations

You can perform standard arithmetic operations (addition, subtraction, multiplication, and division) on elements within the field.

```rust
let a = PrimeField::new(BigUint::from(5u32), prime.clone());
let b = PrimeField::new(BigUint::from(3u32), prime.clone());

let sum = a.clone() + b.clone();          // (5 + 3) % 17 = 8
let difference = a.clone() - b.clone();   // (5 - 3) % 17 = 2
let product = a.clone() * b.clone();      // (5 * 3) % 17 = 15
let quotient = a.clone() / b.clone();     // (5 * 3^-1) % 17 = 5 * 6 % 17 = 13 (since 3^-1 % 17 = 6)
```

### Modular Inversion

Compute the multiplicative inverse of an element, which is essential for division operations within the field.

```rust
let inverse = a.inverse();  // 5^-1 % 17 = 7 (since 5 * 7 % 17 = 1)
```

### Power Computations

Raise an element to a specific power within the field.

```rust
let power = a.pow(3);  // 5^3 % 17 = 125 % 17 = 6
```

### Zero and Order of the Field

You can create a zero element within the field and retrieve the order of the field (the prime).

```rust
let zero = PrimeField::zero(prime.clone());  // Creates a zero element with the same prime field
let order = a.order();                      // Returns the prime of the field, which is 17
```

### Displaying Field Elements

The `Display` trait is implemented for easy and readable output of field elements.

```rust
println!("Field element: {}", a);  // Prints: Field element: 5
```

### Example Usage

```rust
use num_bigint::BigUint;
use finite_field::PrimeField;

fn main() {
    let prime = BigUint::from(17u32);
    let num = BigUint::from(20u32);
    let field_element = PrimeField::new(num, prime.clone());

    println!("Field element: {}", field_element);

    let a = PrimeField::new(BigUint::from(5u32), prime.clone());
    let b = PrimeField::new(BigUint::from(3u32), prime.clone());

    println!("Sum: {}", a.clone() + b.clone());
    println!("Subtract: {}", a.clone() - b.clone());
    println!("Product: {}", a.clone() * b.clone());
    println!("Quotient: {}", a.clone() / b.clone());
    println!("Inverse: {}", a.inverse());
    println!("Power: {}", a.pow(3));
    println!("Zero element: {}", PrimeField::zero(prime.clone()));
    println!("Field order: {}", a.order());
}
```

## Elliptic Curve and ECDSA

In this project, I focus on implementing the secp256k1 elliptic curve. The implementation leverages my `finite_field` project to handle elements over finite fields with the prime modulus being the secp256k1 prime. This setup not only supports the secp256k1 curve but also allows for the extension to other elliptic curves based on the same foundational point structure.

To use `elliptic_curve` package, you need to clone it and config `Cargo.toml`

```rust
[dependencies]
elliptic_curve = { path = "../elliptic_curve" }
```

### Usage for Secp256k1

Here's a guide on how to use the `Secp256k1` struct to perform various operations.

1. **Initialize the Curve:**

   ```rust
   use elliptic_curve::secp256k1::*;
   use num_bigint::BigUint;
   use num_traits::Num;

   fn main() {
       let secp256k1 = Secp256k1::new();
       println!("Secp256k1 Curve Initialized");
   }
   ```

2. **Get the Generator Point:**

   ```rust
   let g = secp256k1.g();
   println!("Generator Point: {:?}", g);
   ```

3. **Create a Custom Point:**

   ```rust
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
   ```

4. **Lift an X Coordinate to a Point:**

   ```rust
   let secp256k1 = Secp256k1::new();
   let x = BigUint::from_str_radix(
       "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
       16,
   )
   .unwrap();
   let point = secp256k1.lift_x(&x);
   println!("Lifted Point: {:?}", point);
   ```

5. **Example Usage**

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

### Usage for ECDSA

The `Ecdsa` struct provides functionality for creating an ECDSA instance, signing messages, and verifying signatures using the secp256k1 elliptic curve. Below is a guide on how to use these features:

### 1. **Creating an ECDSA Instance**

To create a new `Ecdsa` instance, you need to generate a private key (`d`). The `Ecdsa` struct will then derive the corresponding public key.

```rust
use num_bigint::BigUint;
use rand::Rng;
use elliptic_curve::ecdsa::*;

fn main() {
    let mut rng = rand::thread_rng();
    let d: BigUint = rng.sample(num_bigint::RandomBits::new(256));

    // Create a new ECDSA instance with the private key d
    let ecdsa = Ecdsa::new(d);
}
```

### 2. **Hashing a Message**

To hash a message using the `Ecdsa` instance, use the `hash` method. This will compute the SHA-256 hash of the message and return it as a `BigUint`.

```rust
let message = "Hello, world!";
let hash_value = ecdsa.hash(message);
println!("Hash: {}", hash_value);
```

### 3. **Signing a Message**

To sign a message, use the `sign` method. It will generate a digital signature `(r, s)` for the given message

```rust
let (r, s) = ecdsa.sign(message);
println!("Signature: (r: {}, s: {})", r, s);
```

### 4. **Verifying a Signature**

To verify a signature, use the `verify` method with the message and the signature `(r, s)`. It will return `true` if the signature is valid, or `false` otherwise.

```rust
let is_valid = ecdsa.verify(message, &r, &s);
if is_valid {
    println!("Signature is valid.");
} else {
    println!("Signature is invalid.");
}
```

### Example Usage

Here is a complete example demonstrating how to create an ECDSA instance, sign a message, and verify the signature:

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

### Running Tests

To ensure that your implementation is correct, you can run the tests defined in the `tests` module:

```
cargo test
```
