# Zero-Knowledge Proofs

Complete Foundation Notes: From Primitives to Proof Systems

## The One Problem ZK Solves

Everything in these notes exists to answer one question:

- How can you prove that you know something, without revealing what you know?

A zero-knowledge proof lets a prover convince a verifier that a statement is true while hiding the secret witness.

## Core ZK Intuition

Imagine you can see a hidden picture in a magic-eye image and need to prove to a friend that you see it, without pointing or describing it. ZK proofs are the mathematical version of this.

## 1. Finite Fields (Galois Fields)

### What is a Field?

A field is a set with operations +, -, *, / where results stay in the set.

### Finite Fields

- GF(p) (or Fp or Z/pZ) has p elements where p is prime.
- Arithmetic is modulo p.

### Example

In GF(7):

- `3 + 5 = 8 mod 7 = 1`
- `4 * 4 = 16 mod 7 = 2`
- `5 - 6 = -1 mod 7 = 6`
- `3/4 = 3 * inv(4) = 3 * 2 = 6`

### Why Primes Only?

If p is composite, some elements lack inverses; field axioms fail.

### Extension Fields

- GF(p^n) extends GF(p), used in elliptic-curve pairings like BLS12-381.

### ZK Need

Finite fields provide precise, bounded computation for polynomial equations and commitments.

---

## 2. Discrete Logarithm Problem

- Given `(g, p, y)` with `y = g^x mod p`, find `x`.
- Hard for large p.

### Example (small)

- `g=3`, `p=17`, `x=5` => `y=5`.
- Reverse search is expensive for real-size primes.

### ZK Connection

Discrete log hardness underpins Schnorr proofs, KZG, and many ZK systems.

---

## 3. Primitive Root (Generator)

- A primitive root g mod p generates all nonzero residues.
- Example mod 7: 3 is a primitive root, 2 is not.

---

## 4. Fermat's Little Theorem

- If p prime and gcd(a,p)=1, then `a^(p-1) ≡ 1 (mod p)`.
- Inverse: `a^(p-2) ≡ a^-1 (mod p)`.

---

## 5. Chinese Remainder Theorem (CRT)

- For pairwise coprime moduli, a set of congruences has a unique solution mod product.
- Useful in RSA optimization, NTT, MPC, ZK polynomial work.

---

## 6. RSA Cryptosystem

- Keygen: choose primes p,q; n=pq; φ(n)=(p-1)(q-1); choose e; find d.
- Encrypt/decrypt: `C=M^e mod n`, `M=C^d mod n`.
- Secure from factoring.

---

## 7. Diffie-Hellman Key Exchange

- Shared secret from `g^a` and `g^b` without leaking a,b.
- Foundation for Sigma protocols and interactive ZK.

---

## 8. Modular Inverse

- `x` s.t. `a*x ≡ 1 (mod m)` when gcd(a,m)=1.
- Compute by Extended Euclidean or Fermat (if prime modulus).

---

## 9. Elliptic Curves

- `y^2 = x^3 + ax + b` (non-singular).
- Group law: point addition, doubling, scalar mult.
- ECDLP hard; modern ZK uses curves like BN254, BLS12-381, Pasta.

---

## 10. Polynomials and Lagrange Interpolation

- Degree d polynomial determined by d+1 points.
- Lagrange basis: `L_i(x) = Π_{j≠i} (x-x_j)/(x_i-x_j)`.
- Core of PLONK/arith circuits and Schwartz-Zippel soundness.

---

## 11. Polynomial GCD and Linear Combinations

- Polynomial Euclidean algorithm and Bezout identity.
- R1CS constraints are linear combos: `(a·w)*(b·w)=c·w`.
- Vanishing polynomial `Z_H(x)` checks all gate points.

---

## 12. AES and Block Ciphers

- AES operates in GF(2^8) with SubBytes, ShiftRows, MixColumns, AddRoundKey.
- ZK circuits for AES are expensive yet used in proofs about encrypted data.

---

## 13. Polynomial Commitments and Trusted Setup (KZG)

- KZG commit: `C = P(τ)*G` with SRS powers of τ.
- Evaluation proof: `π = Q(τ)*G` where `Q(x) = (P(x)-y)/(x-z)`.
- Requires trusted setup (Powers-of-Tau ceremony).

---

## 14. Arithmetic Circuits and Representation

- Arithmetic circuit = DAG of add/mul gates.
- SNARKs encode computation as a polynomial/algebraic relation and prove it succinctly.

