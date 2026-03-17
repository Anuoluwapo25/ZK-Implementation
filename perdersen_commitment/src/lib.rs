use std::fmt;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, MulAssign};

const P: u64 = 1_000_000_007;

// The multiplicative group GF(P)* has order P-1 (Fermat's little theorem).
// Messages live in GF(P) — field arithmetic, mod P.
// Blindings are EXPONENTS — they live in Z/(P-1), not Z/P.
// These two moduli are different, so we need separate arithmetic for each.
const GROUP_ORDER: u128 = P as u128 - 1;   // = P - 1

pub const G: FieldElement = FieldElement(2);
pub const H: FieldElement = FieldElement(3);

// ── Scalar helpers: blinding arithmetic (mod GROUP_ORDER) ────────────────────
fn sc_add(a: u64, b: u64) -> u64 { ((a as u128 + b as u128)             % GROUP_ORDER) as u64 }
fn sc_sub(a: u64, b: u64) -> u64 { ((a as u128 + GROUP_ORDER - b as u128) % GROUP_ORDER) as u64 }
fn sc_mul(a: u64, b: u64) -> u64 { ((a as u128 * b as u128)             % GROUP_ORDER) as u64 }

// ── Field element: GF(P) ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement(u64);

impl FieldElement {
    pub const ZERO: Self = Self(0);
    pub const ONE:  Self = Self(1);

    pub fn new(v: u64) -> Self { Self(v % P) }

    pub fn inner(self) -> u64 { self.0 }

    pub fn pow(self, mut e: u64) -> Self {
        let mut base = self.0;
        let mut result = 1u64;
        while e > 0 {
            if e & 1 == 1 { result = mul128(result, base); }
            e >>= 1;
            base = mul128(base, base);
        }
        Self(result)
    }

    fn inverse(self) -> Self {
        assert!(self.0 != 0, "no inverse for zero");
        self.pow(P - 2)          // Fermat: a^(p-2) = a^(-1) mod p
    }
}

fn mul128(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

impl Add  for FieldElement { type Output = Self; fn add(self, r: Self) -> Self { let s = self.0 + r.0; Self(if s >= P { s - P } else { s }) } }
impl Sub  for FieldElement { type Output = Self; fn sub(self, r: Self) -> Self { Self(if self.0 >= r.0 { self.0 - r.0 } else { self.0 + P - r.0 }) } }
impl Mul  for FieldElement { type Output = Self; fn mul(self, r: Self) -> Self { Self(mul128(self.0, r.0)) } }
impl Neg  for FieldElement { type Output = Self; fn neg(self) -> Self { if self.0 == 0 { self } else { Self(P - self.0) } } }
impl std::ops::Div for FieldElement { type Output = Self; fn div(self, r: Self) -> Self { self * r.inverse() } }
impl AddAssign for FieldElement { fn add_assign(&mut self, r: Self) { *self = *self + r; } }
impl MulAssign for FieldElement { fn mul_assign(&mut self, r: Self) { *self = *self * r; } }
impl fmt::Display for FieldElement { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) } }

// ── Pedersen commitment ───────────────────────────────────────────────────────
//
//  C = G^m * H^r  in GF(P)*
//
//  The inner FieldElement of Commitment stores a *group element* (value in GF(P)).
//  The blinding in Opening stores a *scalar exponent* (value in Z/(P-1)).
//  We keep both as FieldElement for simplicity, but combine them differently:
//    messages  → + - * via FieldElement ops (mod P)
//    blindings → sc_add / sc_sub / sc_mul           (mod P-1)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Commitment(pub FieldElement);

#[derive(Debug, Clone, Copy)]
pub struct Opening {
    pub message:  FieldElement,   // m — the secret value
    pub blinding: FieldElement,   // r — the random mask (an exponent, lives in Z/(P-1))
}

pub struct Pedersen;

impl Pedersen {
    /// C = G^m * H^r
    pub fn commit(m: FieldElement, r: FieldElement) -> (Commitment, Opening) {
        let c = G.pow(m.inner()) * H.pow(r.inner());
        (Commitment(c), Opening { message: m, blinding: r })
    }

    /// Recompute C from the opening and compare.
    pub fn verify(c: &Commitment, o: &Opening) -> bool {
        let (recomputed, _) = Self::commit(o.message, o.blinding);
        recomputed == *c
    }

    /// `bool::then_some(v)`:
    ///   true  → Some(v)
    ///   false → None
    /// Equivalent to: if verify { Some(message) } else { None }
    pub fn open(c: &Commitment, o: &Opening) -> Option<FieldElement> {
        Self::verify(c, o).then_some(o.message)
    }

    // ── Homomorphic operations ────────────────────────────────────────────────
    // C1 * C2  =  G^(m1+m2) * H^(r1+r2)  — no secrets needed.

    pub fn add(c1: Commitment, c2: Commitment) -> Commitment {
        Commitment(c1.0 * c2.0)
    }
    pub fn add_openings(o1: Opening, o2: Opening) -> Opening {
        Opening {
            message:  o1.message + o2.message,                         // mod P
            blinding: FieldElement(sc_add(o1.blinding.0, o2.blinding.0)), // mod P-1
        }
    }

    pub fn sub(c1: Commitment, c2: Commitment) -> Commitment {
        Commitment(c1.0 * c2.0.inverse())
    }
    pub fn sub_openings(o1: Opening, o2: Opening) -> Opening {
        Opening {
            message:  o1.message - o2.message,                         // mod P
            blinding: FieldElement(sc_sub(o1.blinding.0, o2.blinding.0)), // mod P-1
        }
    }

    /// C^k  =  G^(k*m) * H^(k*r)
    pub fn scale(c: Commitment, k: FieldElement) -> Commitment {
        Commitment(c.0.pow(k.inner()))
    }
    pub fn scale_opening(o: Opening, k: FieldElement) -> Opening {
        Opening {
            message:  o.message * k,                                   // mod P
            blinding: FieldElement(sc_mul(o.blinding.0, k.0)),            // mod P-1
        }
    }

    /// For confidential transactions: combine two blindings correctly (mod P-1).
    /// Use this instead of FieldElement `+` when summing blindings in your application.
    pub fn add_blindings(r1: FieldElement, r2: FieldElement) -> FieldElement {
        FieldElement(sc_add(r1.0, r2.0))
    }

    /// Confidential-tx check: product of input commitments == product of outputs.
    pub fn verify_sum(inputs: &[Commitment], outputs: &[Commitment]) -> bool {
        let fold = |cs: &[Commitment]| cs.iter().fold(FieldElement::ONE, |acc, c| acc * c.0);
        fold(inputs) == fold(outputs)
    }
}
