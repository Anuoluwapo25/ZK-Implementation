// ============================================================
//  src/field.rs — Finite Field Arithmetic over GF(p)
// ============================================================
//
// WHY DO WE NEED A FINITE FIELD?
// ================================
// All MPC operations happen "modulo a prime p".  This guarantees:
//   • All values stay bounded (no overflow in the final result)
//   • Division always exists (every non-zero element has an inverse)
//   • Information-theoretic security: shares are uniformly random
//
// We use p = 2^61 − 1  (a Mersenne prime) because:
//   • It is large enough to hold everyday secrets (salaries, IDs, …)
//   • Products a·b < 2^122 fit inside Rust's i128 (max ≈ 2^127)
//   • Modular reduction is fast on 64-bit CPUs

/// p = 2^61 − 1  (Mersenne prime)
pub const PRIME: i128 = 2_305_843_009_213_693_951;

/// (a + b) mod p
pub fn add(a: i128, b: i128) -> i128 {
    (a + b).rem_euclid(PRIME)
}

/// (a − b) mod p   (result is always in [0, p))
pub fn sub(a: i128, b: i128) -> i128 {
    (a - b).rem_euclid(PRIME)
}

/// (a × b) mod p
/// Safe because p < 2^61, so a·b < 2^122, well within i128 range.
pub fn mul(a: i128, b: i128) -> i128 {
    (a * b).rem_euclid(PRIME)
}

/// Extended Euclidean Algorithm.
/// Returns (gcd, x, y) such that  a·x + b·y = gcd(a, b).
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

/// Modular inverse: returns x such that  (a · x) mod p = 1.
/// Used to implement division in GF(p).
pub fn mod_inverse(a: i128) -> i128 {
    let (_, x, _) = extended_gcd(a.rem_euclid(PRIME), PRIME);
    x.rem_euclid(PRIME)
}

/// (a / b) mod p  =  a · b^{−1} mod p
pub fn div(a: i128, b: i128) -> i128 {
    mul(a, mod_inverse(b))
}

// ─────────────────────────── tests ───────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sub_roundtrip() {
        let a = 123_456_789i128;
        let b = 987_654_321i128;
        assert_eq!(sub(add(a, b), b), a);
    }

    #[test]
    fn mul_div_roundtrip() {
        let a = 42i128;
        let b = 7i128;
        assert_eq!(div(mul(a, b), b), a);
    }

    #[test]
    fn inverse_identity() {
        let a = 99_999i128;
        assert_eq!(mul(a, mod_inverse(a)), 1);
    }
}
