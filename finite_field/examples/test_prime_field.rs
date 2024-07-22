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
