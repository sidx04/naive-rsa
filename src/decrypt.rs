use num::BigUint;

use crate::utils;

pub fn decrypt(encrypted_message: BigUint, d: BigUint, n: BigUint) -> BigUint {
    let message = utils::modular_exponentiation(&encrypted_message, &d, &n);
    return message.to_biguint().unwrap();
}
