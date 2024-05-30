use num::{BigInt, BigUint};
use num_bigint::ToBigInt;

use crate::utils;

pub fn encrypt(msg: BigUint, e: BigUint, n: BigUint) -> BigInt {
    let enc_msg = utils::modular_exponentiation(&msg, &e, &n);

    return enc_msg.to_bigint().unwrap();
}
