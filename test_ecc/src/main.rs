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
