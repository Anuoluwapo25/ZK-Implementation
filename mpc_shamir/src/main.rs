// ============================================================
//  main.rs — MPC Demo using Shamir's Secret Sharing
// ============================================================
//
//  Three demos, each teaching a different concept:
//
//  Demo 1 — SECURE SUM OF SALARIES
//    Three employees compute their total salary without anyone
//    learning the others' individual salaries.
//    Technique: secure addition (zero communication, fully local).
//
//  Demo 2 — SECURE MULTIPLICATION (Beaver Triples)
//    Two parties compute  x · y  without revealing x or y.
//    Technique: Beaver triple protocol (2 broadcast rounds).
//
//  Demo 3 — PRIVACY GUARANTEE
//    Visual proof that individual shares look random and reveal
//    nothing about the secret.

use mpc_shamir::{party::Party, protocol};

fn sep() { println!("{}", "─".repeat(60)); }
fn sep2() { println!("{}", "═".repeat(60)); }

fn main() {
    demo_secure_salary_sum();
    println!();
    demo_secure_multiplication();
    println!();
    demo_privacy_guarantee();
}

// ============================================================
//  DEMO 1: Secure Average Salary
// ============================================================
fn demo_secure_salary_sum() {
    sep2();
    println!("  DEMO 1 — Secure Sum & Average of Salaries");
    sep2();
    println!();
    println!("Three employees want to know their AVERAGE salary.");
    println!("Privacy rule: nobody learns any individual salary.");
    println!();

    // ── Setup ─────────────────────────────────────────────────
    // 3 parties, threshold = 2  (any 2-of-3 shares can reconstruct)
    let t = 2;
    let mut parties = vec![
        Party::new(1, "Alice"),
        Party::new(2, "Bob"),
        Party::new(3, "Charlie"),
    ];

    // Private inputs — only the simulator (us) can see all three
    let alice_salary:   i128 = 95_000;
    let bob_salary:     i128 = 82_000;
    let charlie_salary: i128 = 110_000;

    let true_total = alice_salary + bob_salary + charlie_salary;
    let true_avg   = true_total / 3;

    println!("[Simulator view — hidden from individual parties]");
    println!("  Alice's salary:   ${alice_salary}");
    println!("  Bob's salary:     ${bob_salary}");
    println!("  Charlie's salary: ${charlie_salary}");
    println!();

    // ── Step 1: Each party secret-shares their salary ─────────
    // Alice splits her salary into 3 shares and sends:
    //   share 1 → herself      (private channel)
    //   share 2 → Bob          (private channel)
    //   share 3 → Charlie      (private channel)
    // Bob and Charlie do the same for their salaries.
    // After this step no party knows another's salary — only a
    // random-looking share of it.
    println!("Step 1  ──  Each party shares their salary as Shamir shares.");
    protocol::input_secret(&mut parties, "alice",   alice_salary,   t);
    protocol::input_secret(&mut parties, "bob",     bob_salary,     t);
    protocol::input_secret(&mut parties, "charlie", charlie_salary, t);
    println!("  Shares held by each party after distribution:");
    for p in &parties {
        println!("    {} → alice_share={:>22}, bob_share={:>22}, charlie_share={:>22}",
            p.name,
            p.get_value("alice"),
            p.get_value("bob"),
            p.get_value("charlie"),
        );
    }
    println!("  (These numbers look random — they reveal nothing alone.)");
    println!();

    // ── Step 2: Local addition — zero communication! ──────────
    // Each party just adds their three share values together.
    // No communication occurs.  The sum of the shares IS a valid
    // share of the sum of the secrets, because Shamir sharing is
    // linear.
    println!("Step 2  ──  Each party LOCALLY adds their three shares.");
    println!("  (No messages sent between parties — pure local arithmetic!)");
    protocol::add(&mut parties, "ab",    "alice",   "bob");
    protocol::add(&mut parties, "total", "ab",      "charlie");
    println!("  Each party now holds a share of the total sum:");
    for p in &parties {
        println!("    {} → total_share = {:>22}", p.name, p.get_value("total"));
    }
    println!();

    // ── Step 3: Open (reconstruct) the total ──────────────────
    println!("Step 3  ──  Parties combine their shares to reveal the total.");
    let computed_total = protocol::open(&parties, "total", t);
    let computed_avg   = computed_total / 3;

    println!("  Computed total : ${computed_total}");
    println!("  Expected total : ${true_total}  ✓ match={}", computed_total == true_total);
    println!();
    println!("  Computed average : ${computed_avg}");
    println!("  Expected average : ${true_avg}  ✓ match={}", computed_avg == true_avg);
    println!();
    println!("Result: All three employees learn the average salary.");
    println!("        Nobody learned anyone else's individual salary.");
}

// ============================================================
//  DEMO 2: Secure Multiplication with Beaver Triples
// ============================================================
fn demo_secure_multiplication() {
    sep2();
    println!("  DEMO 2 — Secure Multiplication (Beaver Triples)");
    sep2();
    println!();
    println!("Alice holds x, Bob holds y.");
    println!("Goal: compute x · y without either revealing their value.");
    println!();

    let t = 2;
    let mut parties = vec![
        Party::new(1, "Alice"),
        Party::new(2, "Bob"),
        Party::new(3, "Charlie"), // needed as a third share-holder
    ];

    let x: i128 = 17;
    let y: i128 = 19;
    let expected = x * y;

    println!("[Simulator view]  x = {x},  y = {y},  expected x·y = {expected}");
    println!();

    // ── Offline / preprocessing phase ─────────────────────────
    println!("OFFLINE PHASE  ──  Trusted dealer generates Beaver triple.");
    println!("  The dealer picks random a, b ∈ GF(p),  computes c = a·b,");
    println!("  then secret-shares all three.  Nobody learns a, b, or c.");
    let triple = protocol::generate_beaver_triple(
        &mut parties, "ba", "bb", "bc", t,
    );
    println!("  Triple [a], [b], [c] distributed.  a·b = c  (verified by dealer only).");
    println!();

    // ── Online phase ──────────────────────────────────────────
    println!("ONLINE PHASE  ──  Parties share inputs and run the protocol.");

    // Share inputs
    protocol::input_secret(&mut parties, "x", x, t);
    protocol::input_secret(&mut parties, "y", y, t);

    println!("  Round 1 (local):  each party computes [ε]=[x]-[a]  and  [δ]=[y]-[b].");
    println!("  Round 1 (broadcast):  all parties reveal ε and δ.");
    println!("    (ε and δ are masked by fresh randomness — they're safe to broadcast.)");
    println!("  Round 2 (local):  each party computes [z]_i = [c]_i + ε·[b]_i + δ·[a]_i");
    println!("    (party 1 also adds the public constant ε·δ).");

    protocol::mul(&mut parties, "product", "x", "y", &triple, t);

    let result = protocol::open(&parties, "product", t);
    println!();
    println!("  Computed x · y = {result}");
    println!("  Expected:        {expected}   ✓ match={}", result == expected);
    println!();

    // Bonus: show that multiplication gates can be chained
    println!("Bonus: compute x · y · 2  using scalar_mul on the result.");
    protocol::scalar_mul(&mut parties, "double_product", "product", 2);
    let result2 = protocol::open(&parties, "double_product", t);
    println!("  Computed x · y · 2 = {result2}");
    println!("  Expected:             {}   ✓ match={}", expected * 2, result2 == expected * 2);
}

// ============================================================
//  DEMO 3: Privacy Guarantee
// ============================================================
fn demo_privacy_guarantee() {
    sep2();
    println!("  DEMO 3 — Privacy Guarantee");
    sep2();
    println!();
    println!("Visual proof: individual shares reveal nothing about the secret.");
    println!();

    let t = 3;   // need 3-of-5 shares
    let mut parties: Vec<Party> = (1..=5)
        .map(|i| Party::new(i, &format!("Party{i}")))
        .collect();

    let secret: i128 = 1337;
    protocol::input_secret(&mut parties, "secret", secret, t);

    println!("Secret value  : {secret}");
    println!("Threshold k   : {t}  (need {t} of 5 shares to reconstruct)");
    println!("Polynomial degree : {}  (k−1 = {})", t - 1, t - 1);
    println!();
    println!("Shares distributed to each party:");
    for p in &parties {
        println!("  {:8}  →  y = {:>22}", p.name, p.get_value("secret"));
    }
    println!();
    println!("Reconstruction attempts:");

    // Exact threshold — should succeed
    let r3 = protocol::open(&parties, "secret", 3);
    println!("  With {} shares (= threshold):   {} {}",
        3, r3, if r3 == secret { "✓ CORRECT" } else { "✗ WRONG" });

    // All shares — should succeed
    let r5 = protocol::open(&parties, "secret", 5);
    println!("  With 5 shares (= all):          {} {}",
        r5, if r5 == secret { "✓ CORRECT" } else { "✗ WRONG" });

    // Below threshold — should give wrong answer
    let r2 = protocol::open(&parties, "secret", 2);
    println!("  With {} shares (< threshold):   {} {}",
        2, r2, if r2 != secret { "✗ WRONG  (expected — privacy holds!)" } else { "✓ (got lucky, try again)" });

    println!();
    sep();
    println!();
    println!("Key takeaways:");
    println!();
    println!("  1. ADDITION IS FREE:  [x+y] computed with zero communication.");
    println!("     Each party adds their own shares locally.  That's it.");
    println!();
    println!("  2. MULTIPLICATION COSTS 2 ROUNDS:  the Beaver triple protocol");
    println!("     broadcasts ε = x−a and δ = y−b.  Because a, b are random,");
    println!("     ε and δ reveal nothing about x and y.");
    println!();
    println!("  3. PRIVACY IS PERFECT:  with fewer than k shares, the adversary");
    println!("     sees a uniformly random-looking value — completely independent");
    println!("     of the actual secret.  This is information-theoretic security.");
    println!();
    println!("  4. THRESHOLD ARITHMETIC STILL WORKS:  the share of [x+y] has the");
    println!("     same threshold as the shares of [x] and [y].  You never need");
    println!("     to reveal the secrets to add them.");
}

