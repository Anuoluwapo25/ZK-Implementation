use perdersen_commitment::{FieldElement, Opening, Pedersen};

fn rnd() -> FieldElement {
    FieldElement::new(rand::random::<u64>())
}

fn section(title: &str) {
    println!("\n{}", "─".repeat(56));
    println!("  {}", title);
    println!("{}", "─".repeat(56));
}

fn main() {
    println!("Pedersen Commitments — GF(p) field-first implementation");

    // ── 1. Commit / Verify / Open ─────────────────────────────────
    section("1. Commit → Verify → Open");

    let m = FieldElement::new(42);
    let r = rnd();                          // random blinding factor
    let (c, o) = Pedersen::commit(m, r);

    println!("message:    {}", m);
    println!("blinding:   {}", r);          // different every run
    println!("commitment: {}", c.0);
    println!("verify(correct):  {}", Pedersen::verify(&c, &o));

    let fake = Opening { message: FieldElement::new(43), blinding: r };
    println!("verify(wrong m):  {}", Pedersen::verify(&c, &fake));

    match Pedersen::open(&c, &o) {
        Some(v) => println!("open(): m = {} ✓", v),
        None    => println!("open(): invalid"),
    }

    // ── 2. Homomorphic Addition ───────────────────────────────────
    section("2. Homomorphic Addition  commit(30) + commit(12)");

    let (c1, o1) = Pedersen::commit(FieldElement::new(30), rnd());
    let (c2, o2) = Pedersen::commit(FieldElement::new(12), rnd());
    let c_sum = Pedersen::add(c1, c2);
    let o_sum = Pedersen::add_openings(o1, o2);

    println!("verify sum (→ 42): {}", Pedersen::verify(&c_sum, &o_sum));
    println!("revealed message:  {}", o_sum.message);

    // ── 3. Homomorphic Subtraction ────────────────────────────────
    section("3. Homomorphic Subtraction  commit(100) - commit(40)");

    let (c3, o3) = Pedersen::commit(FieldElement::new(100), rnd());
    let (c4, o4) = Pedersen::commit(FieldElement::new(40),  rnd());
    let c_diff = Pedersen::sub(c3, c4);
    let o_diff = Pedersen::sub_openings(o3, o4);

    println!("verify diff (→ 60): {}", Pedersen::verify(&c_diff, &o_diff));
    println!("revealed message:   {}", o_diff.message);

    // ── 4. Scalar Scaling ─────────────────────────────────────────
    section("4. Scale  commit(7) × 5");

    let (c5, o5) = Pedersen::commit(FieldElement::new(7), rnd());
    let k = FieldElement::new(5);
    let c5k = Pedersen::scale(c5, k);
    let o5k = Pedersen::scale_opening(o5, k);

    println!("verify scaled (→ 35): {}", Pedersen::verify(&c5k, &o5k));
    println!("revealed message:     {}", o5k.message);

    // ── 5. Confidential Transaction ───────────────────────────────
    // The blindings must satisfy: r_in = r_out1 + r_out2
    // so we generate r_out1 and r_out2 randomly, then derive r_in.
    section("5. Confidential Transaction  150 → 100 + 50");

    let r_bob = rnd();
    let r_chg = rnd();
    // Blindings are exponents (live in Z/(P-1)), so use add_blindings, not +
    let r_in  = Pedersen::add_blindings(r_bob, r_chg);

    let (c_in,  _) = Pedersen::commit(FieldElement::new(150), r_in);
    let (c_bob, _) = Pedersen::commit(FieldElement::new(100), r_bob);
    let (c_chg, _) = Pedersen::commit(FieldElement::new(50),  r_chg);

    println!("balanced: {}", Pedersen::verify_sum(&[c_in], &[c_bob, c_chg]));
    println!("(amounts hidden — verifier sees only commitments)");

    println!("\nAll checks passed.");
}
