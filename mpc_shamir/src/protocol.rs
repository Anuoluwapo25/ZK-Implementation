// ============================================================
//  src/protocol.rs — MPC Protocols
// ============================================================
//
// THE CORE INSIGHT OF SHAMIR-BASED MPC
// ======================================
// Suppose every party i holds a share of secret x (call it [x]ᵢ)
// and a share of secret y (call it [y]ᵢ).
//
//   ADDITION  is FREE (zero communication):
//     [x + y]ᵢ  =  [x]ᵢ + [y]ᵢ          ← local, no messages
//
//   SCALAR MULTIPLICATION is also FREE:
//     [c · x]ᵢ  =  c · [x]ᵢ             ← local, no messages
//
//   MULTIPLICATION of two shared values requires communication:
//     [x · y]ᵢ  = ???                    ← needs Beaver triples
//
// Why?  Because addition of degree-(k−1) polynomials gives another
// degree-(k−1) polynomial — shares stay valid.  But multiplying two
// degree-(k−1) polynomials gives degree-2(k−1), which would double
// the threshold.  Beaver triples let us "correct" the degree.

use rand::Rng;
use crate::{field, shamir};
use crate::party::Party;

// ============================================================
//  SETUP
// ============================================================

/// Distribute a secret as Shamir shares to all parties.
///
/// In a real deployment the secret holder would:
///   1. Run `shamir::split` locally.
///   2. Send share i to party i over an encrypted private channel.
///
/// We simulate that by calling `party.receive_share` for each party.
pub fn input_secret(parties: &mut [Party], label: &str, secret: i128, threshold: usize) {
    let shares = shamir::split(secret, parties.len(), threshold);
    for (party, share) in parties.iter_mut().zip(shares.iter()) {
        party.receive_share(label, share.y);
    }
}

// ============================================================
//  COMMUNICATION-FREE OPERATIONS  ("free" in MPC terms)
// ============================================================

/// SECURE ADDITION  —  compute [z] = [x] + [y]  with NO communication.
///
/// Each party independently adds their two share values:
///   [z]ᵢ  =  [x]ᵢ + [y]ᵢ  (mod p)
///
/// This works because polynomial addition is term-wise:
///   P_x(i) + P_y(i) = P_{x+y}(i)
pub fn add(parties: &mut [Party], result: &str, x: &str, y: &str) {
    for p in parties.iter_mut() {
        let z = field::add(p.get_value(x), p.get_value(y));
        p.receive_share(result, z);
    }
}

/// SECURE SUBTRACTION  —  compute [z] = [x] − [y]  with NO communication.
pub fn sub(parties: &mut [Party], result: &str, x: &str, y: &str) {
    for p in parties.iter_mut() {
        let z = field::sub(p.get_value(x), p.get_value(y));
        p.receive_share(result, z);
    }
}

/// SECURE SCALAR MULTIPLY  —  compute [z] = c · [x]  with NO communication.
///
/// Each party just multiplies their share by the public constant c:
///   [z]ᵢ  =  c · [x]ᵢ  (mod p)
pub fn scalar_mul(parties: &mut [Party], result: &str, x: &str, c: i128) {
    for p in parties.iter_mut() {
        let z = field::mul(c, p.get_value(x));
        p.receive_share(result, z);
    }
}

// ============================================================
//  RECONSTRUCTION  (reveals a shared value to everyone)
// ============================================================

/// OPEN  —  reconstruct the plaintext value of a labeled shared secret.
///
/// In a real protocol, each party broadcasts their share; every party
/// then runs Lagrange interpolation independently.
///
/// We simulate that by collecting `threshold` shares and calling
/// `shamir::reconstruct`.
pub fn open(parties: &[Party], label: &str, threshold: usize) -> i128 {
    let shares: Vec<_> = parties.iter()
        .take(threshold)
        .map(|p| p.get_share(label))
        .collect();
    shamir::reconstruct(&shares)
}

// ============================================================
//  BEAVER TRIPLE MULTIPLICATION  (2 rounds of communication)
// ============================================================
//
// A Beaver triple is a pre-computed secret-shared tuple  ([a], [b], [c])
// where a and b are uniformly random in GF(p) and  c = a · b.
//
// It is generated in an OFFLINE / PREPROCESSING phase before the
// actual computation begins.  (In practice this offline phase can
// be done with oblivious transfer, but here a trusted dealer suffices.)
//
// PROTOCOL to compute  [z] = [x] · [y]  given a Beaver triple:
//
//   ┌─────────────────────────────────────────────────────────┐
//   │  OFFLINE (before computation):                          │
//   │    Dealer samples random a, b ∈ GF(p),  c = a·b        │
//   │    Distributes shares [a], [b], [c] to all parties.     │
//   │                                                         │
//   │  ONLINE (2 broadcast rounds):                           │
//   │                                                         │
//   │  Round 1 (local + broadcast):                           │
//   │    Each party i computes:                               │
//   │      ε_i = [x]_i − [a]_i    (local)                    │
//   │      δ_i = [y]_i − [b]_i    (local)                    │
//   │    All parties broadcast their ε_i and δ_i shares.      │
//   │    Everyone reconstructs ε = x−a  and  δ = y−b.        │
//   │                                                         │
//   │  Round 2 (local):                                       │
//   │    [z]_i = [c]_i + ε·[b]_i + δ·[a]_i + ε·δ            │
//   │            (add ε·δ to EVERY party's share)             │
//   │                                                         │
//   │  WHY DOES THIS WORK?                                    │
//   │    x·y = (ε+a)·(δ+b)                                   │
//   │        = ε·δ + ε·b + δ·a + a·b                         │
//   │        = ε·δ + ε·[b] + δ·[a] + [c]                     │
//   │    Reconstruction: P(0) = Σ [z]_i·L_i(0)               │
//   │      = c + ε·b + δ·a + ε·δ·Σ L_i(0) = x·y  ✓          │
//   │    (Σ L_i(0) = 1  always, for any k-subset)  □         │
//   │                                                         │
//   │  WHY IS IT PRIVATE?                                     │
//   │    ε = x − a   is uniformly random (a is random).       │
//   │    δ = y − b   is uniformly random (b is random).       │
//   │    So broadcasting ε and δ reveals NOTHING about x, y.  │
//   └─────────────────────────────────────────────────────────┘

/// Handle for a pre-computed Beaver triple stored in party memory.
pub struct BeaverTriple {
    pub a_label: String,
    pub b_label: String,
    pub c_label: String,
}

/// Generate and distribute a fresh Beaver triple to all parties.
/// Call this ONCE in the preprocessing phase, before `mul`.
pub fn generate_beaver_triple(
    parties: &mut [Party],
    a_label: &str,
    b_label: &str,
    c_label: &str,
    threshold: usize,
) -> BeaverTriple {
    let mut rng = rand::thread_rng();
    let a: i128 = rng.gen_range(1..field::PRIME);
    let b: i128 = rng.gen_range(1..field::PRIME);
    let c: i128 = field::mul(a, b);   // c = a · b

    // Distribute shares of a, b, c  (the actual values stay secret)
    input_secret(parties, a_label, a, threshold);
    input_secret(parties, b_label, b, threshold);
    input_secret(parties, c_label, c, threshold);

    BeaverTriple {
        a_label: a_label.to_string(),
        b_label: b_label.to_string(),
        c_label: c_label.to_string(),
    }
}

/// SECURE MULTIPLICATION  —  compute [z] = [x] · [y] using a Beaver triple.
///
/// Requires 2 rounds of broadcast communication  (see protocol box above).
pub fn mul(
    parties: &mut [Party],
    result_label: &str,
    x_label: &str,
    y_label: &str,
    triple: &BeaverTriple,
    threshold: usize,
) {
    // Internal labels for the masked differences (not exposed to caller)
    let eps_label = format!("__eps_{result_label}");
    let del_label = format!("__del_{result_label}");

    // ── Round 1 (local): mask x and y ──────────────────────────
    // [ε] = [x] − [a]   and   [δ] = [y] − [b]
    sub(parties, &eps_label, x_label, &triple.a_label);
    sub(parties, &del_label, y_label, &triple.b_label);

    // ── Broadcast: reveal ε and δ  (COMMUNICATION) ─────────────
    // These are safe to reveal: ε = x−a looks uniformly random
    // because a is a fresh random field element.
    let epsilon = open(parties, &eps_label, threshold);
    let delta   = open(parties, &del_label, threshold);

    // ── Round 2 (local): compute share of z ────────────────────
    // [z]_i = [c]_i + ε·[b]_i + δ·[a]_i  +  ε·δ   (for EVERY party i)
    //
    // WHY ADD ε·δ TO EVERY PARTY'S SHARE?
    // ──────────────────────────────────────
    // Lagrange reconstruction computes  P(0) = Σᵢ [z]ᵢ · Lᵢ(0)
    // where the Lagrange basis polynomials satisfy  Σᵢ Lᵢ(0) = 1.
    //
    // If we add the constant ε·δ to EVERY party i's share then:
    //   P_z(0) = ... + ε·δ · Σᵢ Lᵢ(0) = ... + ε·δ · 1 = x·y  ✓
    //
    // If we had added ε·δ to only party 1's share, we would get:
    //   P_z(0) = ... + ε·δ · L₁(0)
    // and L₁(0) depends on which parties reconstruct — NOT 1 in general.
    // (e.g., with parties {1,2,3} and threshold 3,  L₁(0) = 3  ≠ 1.)
    for party in parties.iter_mut() {
        let ai = party.get_value(&triple.a_label);
        let bi = party.get_value(&triple.b_label);
        let ci = party.get_value(&triple.c_label);

        // ε·[b]_i + δ·[a]_i + [c]_i + ε·δ
        let zi = field::add(
            field::add(
                field::add(field::mul(epsilon, bi), field::mul(delta, ai)),
                ci,
            ),
            field::mul(epsilon, delta),  // ← added to ALL parties
        );

        party.receive_share(result_label, zi);
    }
}

// ─────────────────────────── tests ───────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::party::Party;

    fn make_parties(n: usize) -> Vec<Party> {
        (1..=n).map(|i| Party::new(i, &format!("P{i}"))).collect()
    }

    #[test]
    fn secure_add_correctness() {
        let mut parties = make_parties(4);
        let t = 3;
        input_secret(&mut parties, "x", 100, t);
        input_secret(&mut parties, "y", 200, t);
        add(&mut parties, "sum", "x", "y");
        assert_eq!(open(&parties, "sum", t), 300);
    }

    #[test]
    fn secure_scalar_mul_correctness() {
        let mut parties = make_parties(4);
        let t = 3;
        input_secret(&mut parties, "x", 7, t);
        scalar_mul(&mut parties, "result", "x", 6);
        assert_eq!(open(&parties, "result", t), 42);
    }

    #[test]
    fn secure_mul_with_beaver_triple() {
        let mut parties = make_parties(5);
        let t = 3;
        input_secret(&mut parties, "x", 11, t);
        input_secret(&mut parties, "y", 13, t);

        let triple = generate_beaver_triple(&mut parties, "ba", "bb", "bc", t);
        mul(&mut parties, "product", "x", "y", &triple, t);

        assert_eq!(open(&parties, "product", t), 143); // 11 × 13 = 143
    }

    #[test]
    fn add_three_secrets() {
        let mut parties = make_parties(3);
        let t = 2;
        input_secret(&mut parties, "a", 10, t);
        input_secret(&mut parties, "b", 20, t);
        input_secret(&mut parties, "c", 30, t);
        add(&mut parties, "ab",  "a",  "b");
        add(&mut parties, "abc", "ab", "c");
        assert_eq!(open(&parties, "abc", t), 60);
    }
}
