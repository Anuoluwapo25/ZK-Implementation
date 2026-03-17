// ============================================================
//  src/shamir.rs — Shamir's Secret Sharing
// ============================================================
//
// THE BIG PICTURE
// ================
// We want to split a secret S into N "shares" so that:
//   • Any K (or more) shares together reveal S exactly.
//   • Fewer than K shares reveal NOTHING about S.
//
// HOW IT WORKS (the polynomial trick)
// =====================================
//  1. SHARE:
//     • Pick a random polynomial  P(x)  of degree K−1
//       with P(0) = S  (the secret is the constant term).
//       e.g. for K=3:  P(x) = S + a₁·x + a₂·x²
//     • Evaluate P at x = 1, 2, …, N.
//     • Give point  (i, P(i))  to party i.
//
//  2. RECONSTRUCT (Lagrange interpolation):
//     • Collect any K shares: (x₁,y₁), …, (xₖ,yₖ).
//     • There is exactly one polynomial of degree ≤ K−1 through
//       those K points.  Evaluate it at x=0 to recover S.
//
//  WHY IS K−1 THE DEGREE?
//     A polynomial of degree d is determined by d+1 points.
//     We want K points to be "just enough", so degree = K−1.
//     Example: K=3 → degree-2 polynomial (a parabola).
//     With only 2 points you can fit infinitely many parabolas.

use rand::Rng;
use crate::field;

/// A single share: the point  (x, y = P(x))  on the secret polynomial.
/// Each party i holds the share where x = i.
#[derive(Debug, Clone, Copy)]
pub struct Share {
    /// x-coordinate — the party index (1-indexed).
    pub x: i128,
    /// y-coordinate — the secret-shared value  P(x).
    pub y: i128,
}

// ─── SPLIT ───────────────────────────────────────────────────

/// Split `secret` into `n` shares with threshold `k`.
///
/// * `k`  — minimum shares needed to reconstruct (threshold).
/// * `n`  — total shares created  (n ≥ k).
///
/// Returns a Vec of n shares.  The polynomial has degree k−1.
pub fn split(secret: i128, n: usize, k: usize) -> Vec<Share> {
    assert!(k >= 1,  "threshold k must be at least 1");
    assert!(k <= n,  "threshold k cannot exceed total shares n");
    assert!(secret < field::PRIME, "secret must be less than the field prime");

    let mut rng = rand::thread_rng();

    // Build the polynomial  P(x) = secret + a₁x + a₂x² + … + a_{k−1}·x^{k−1}
    // coeffs[0] = secret  (the constant term, i.e. P(0))
    // coeffs[1..k] are random field elements
    let mut coeffs = vec![secret];
    for _ in 1..k {
        coeffs.push(rng.gen_range(0..field::PRIME));
    }

    // Evaluate P at x = 1, 2, …, n
    (1..=n)
        .map(|i| Share {
            x: i as i128,
            y: eval_poly(&coeffs, i as i128),
        })
        .collect()
}

// ─── RECONSTRUCT ─────────────────────────────────────────────

/// Reconstruct the secret P(0) from `k` or more shares.
///
/// Uses Lagrange interpolation at x = 0:
///
///   P(0) = Σⱼ  yⱼ · Lⱼ(0)
///
/// where the j-th Lagrange basis polynomial evaluated at 0 is:
///
///   Lⱼ(0) = Π_{m ≠ j}  (0 − xₘ) / (xⱼ − xₘ)
pub fn reconstruct(shares: &[Share]) -> i128 {
    let mut secret = 0i128;

    for (j, sj) in shares.iter().enumerate() {
        // Compute the Lagrange basis  Lⱼ(0)
        let mut basis = 1i128;
        for (m, sm) in shares.iter().enumerate() {
            if j == m { continue; }
            let num = field::sub(0, sm.x);      // 0 − xₘ  mod p
            let den = field::sub(sj.x, sm.x);   // xⱼ − xₘ mod p
            basis = field::mul(basis, field::div(num, den));
        }
        secret = field::add(secret, field::mul(sj.y, basis));
    }

    secret
}

// ─── HELPERS ─────────────────────────────────────────────────

/// Evaluate  P(x) = coeffs[0] + coeffs[1]·x + coeffs[2]·x² + …
/// using Horner's method:  P(x) = coeffs[0] + x·(coeffs[1] + x·(…))
fn eval_poly(coeffs: &[i128], x: i128) -> i128 {
    let mut result = 0i128;
    for &c in coeffs.iter().rev() {
        result = field::add(field::mul(result, x), c);
    }
    result
}

// ─────────────────────────── tests ───────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_roundtrip() {
        let secret = 123_456_789i128;
        let shares = split(secret, 5, 3);
        assert_eq!(reconstruct(&shares[..3]), secret);
    }

    #[test]
    fn any_k_shares_work() {
        let secret = 987_654_321i128;
        let shares = split(secret, 10, 4);
        // First 4
        assert_eq!(reconstruct(&shares[..4]), secret);
        // Last 4
        assert_eq!(reconstruct(&shares[6..]), secret);
        // Scattered
        let mixed: Vec<Share> = shares.iter().step_by(2).cloned().take(4).collect();
        assert_eq!(reconstruct(&mixed), secret);
    }

    #[test]
    fn below_threshold_gives_wrong_answer() {
        // With only k−1 shares the "reconstruction" gives a random value, not the secret.
        let secret = 42i128;
        let shares = split(secret, 5, 3);
        let bad = reconstruct(&shares[..2]);
        assert_ne!(bad, secret, "Should not reconstruct with fewer than k shares");
    }
}
