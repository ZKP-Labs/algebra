use elliptic_curve::ecdsa::*;
use num_bigint::BigUint;
use rand::Rng;

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
