mod decrypt;
mod encrypt;
mod generate_keys;
mod utils;
use num::{BigUint, FromPrimitive};

const SIZE: usize = 64;

use generate_keys::generate;
fn main() {
    let message = BigUint::from_i128(123234).unwrap();
    let rsa_keys = generate(SIZE);

    let encrypted = encrypt::encrypt(
        message,
        rsa_keys.0.to_biguint().unwrap(),
        rsa_keys.2.to_biguint().unwrap(),
    );
    let decrypted = decrypt::decrypt(
        encrypted.to_biguint().unwrap(),
        rsa_keys.1.to_biguint().unwrap(),
        rsa_keys.2.to_biguint().unwrap(),
    );

    println!("Encrypted message: {}", encrypted);
    println!("Decrypted message: {}", decrypted);
}
