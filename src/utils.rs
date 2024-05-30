use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::traits::{One, Zero};
use rand::Rng;

// The modular_exponentiation() function takes three identical types
// (which get cast to BigInt), and returns a BigInt:
pub fn modular_exponentiation<T: ToBigInt>(n: &T, e: &T, m: &T) -> BigInt {
    let n = n.to_bigint().unwrap();
    let m = m.to_bigint().unwrap();
    let e = e.to_bigint().unwrap();

    // exponent is non-negative
    assert!(e >= Zero::zero());

    // As most modular exponentiations do, return 1 if the exponent is 0:
    if e == Zero::zero() {
        return One::one();
    } else {
        // Now do the modular exponentiation algorithm:
        let mut result: BigInt = One::one();
        let mut base = n % &m;
        let mut exp = e;

        loop {
            // Loop until we can return our result.
            if &exp % 2 == One::one() {
                result *= &base;
                result %= &m;
            }

            if exp == One::one() {
                return result;
            }

            exp /= 2;
            base *= base.clone();
            base %= &m;
        }
    }
}

pub fn get_random_bigint(low: &BigInt, high: &BigInt) -> BigInt {
    if low == high {
        // base case
        return low.clone();
    }

    let middle = (low.clone() + high) / 2.to_bigint().unwrap();

    let go_low: bool = rand::random();

    if go_low {
        return get_random_bigint(low, &middle);
    } else {
        return get_random_bigint(&middle, high);
    }
}

pub fn miller_rabin_prime<T: ToBigInt>(n: &T, k: Option<usize>) -> bool {
    let n = n.to_bigint().unwrap();
    let k = k.unwrap_or(10); // number of times for testing (defaults to 10)

    let zero: BigInt = Zero::zero();
    let one: BigInt = One::one();
    let two: BigInt = 2.to_bigint().unwrap();

    // The call to is_prime() should have already checked this,
    // but check for two, less than two, and multiples of two:
    if n <= one {
        return false;
    } else if n == two {
        return true; // 2 is prime
    } else if n.clone() % &two == Zero::zero() {
        return false; // even number (that's not 2) is not prime
    } else {
        let mut t: BigInt = zero.clone();
        let n_minus_one: BigInt = n.clone() - &one;
        let mut s = n_minus_one.clone();
        while &s % &two == one {
            s /= &two;
            t += &one;
        }

        // Try k times to test if our number is non-prime:
        #[allow(unused_labels)]
        'outer: for _ in 0..k {
            let a = get_random_bigint(&two, &n_minus_one);
            let mut v = modular_exponentiation(&a, &s, &n);
            if v == one {
                continue 'outer;
            }
            let mut i: BigInt = zero.clone();
            'inner: while &i < &t {
                v = (v.clone() * &v) % &n;
                if &v == &n_minus_one {
                    continue 'outer;
                }
                i += &one;
            }
            return false;
        }
        // If we get here, then we have a degree of certainty
        // that n really is a prime number, so return true:
        true
    }
}

pub fn generate_n_bit_prime_number(n: usize) -> BigInt {
    loop {
        let mut rng = rand::thread_rng();

        // Create a BigInt with 1 left-shifted by n bits to represent the maximum possible n-bit number
        let max_value = (BigInt::from(1u64) << n) - 1;

        // Generate a random number within the range [0, max_value)
        let mut bytes = vec![0u8; (n + 7) / 8]; // n bits = (n + 7) / 8 bytes
        rng.fill(&mut bytes[..]);

        // Convert the bytes to a BigInt
        let mut random_number = BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);

        // Ensure the number is within the correct range [0, max_value)
        if random_number > max_value {
            random_number = random_number % (max_value + 1);
        }

        if miller_rabin_prime(&random_number, None) {
            return random_number;
        }
    }
}

pub fn modinv(x: BigInt, y: BigInt) -> BigInt {
    if y == BigInt::one() {
        return BigInt::one();
    }
    let (mut a, mut m, mut x0, mut inv) = (x, y.clone(), BigInt::zero(), BigInt::one());
    while a > BigInt::one() {
        inv -= (&a / &m) * &x0;
        a = &a % &m;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x0, &mut inv)
    }
    if inv < BigInt::zero() {
        inv += y
    }
    inv
}
