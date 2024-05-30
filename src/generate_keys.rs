use num::BigInt;

pub fn generate(size: usize) -> (BigInt, BigInt, BigInt, BigInt) {
    let p = crate::utils::generate_n_bit_prime_number(size);
    let q = crate::utils::generate_n_bit_prime_number(size);
    let p_1: BigInt = p.clone() - 1;
    let q_1: BigInt = q.clone() - 1;

    let n = p.checked_mul(&q).unwrap();
    let phi = p_1.checked_mul(&q_1).unwrap();

    let e = crate::utils::generate_n_bit_prime_number(size * size);
    let d = crate::utils::modinv(e.clone(), phi.clone());
    (e, d, n, phi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::traits::One;

    #[test]
    fn verify_generate() {
        let size = 1024;
        let res = generate(size);

        let one: BigInt = One::one();
        assert_eq!((res.0 * res.1) % res.3, one, "e * d % phi should be 1");
    }
}
