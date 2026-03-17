use shamir_secret_sharing::{Share, split_secret, reconstruct_secret, PRIME};

fn main() {
    println!("--- Shamir's Secret Sharing Demo ---");
    println!("Field Prime: {}\n", PRIME);

    // 1. The secret we want to protect
    let secret: i128 = 133742069; 
    let n = 5; // Total shares to generate
    let k = 3; // Threshold (number of shares needed to reconstruct)

    println!("Original Secret: {}", secret);
    println!("Configuration: (k={}, n={})", k, n);
    println!("  Any {} shares can reconstruct the secret.", k);
    println!("  Fewer than {} shares reveal nothing.\n", k);

    // 2. Split the secret
    println!("Splitting secret into {} shares...", n);
    let shares = split_secret(secret, n, k);
    for (i, share) in shares.iter().enumerate() {
        println!("  Share {}: (x: {}, y: {})", i + 1, share.x, share.y);
    }
    println!("");

    // 3. Reconstruct with exactly K shares
    let subset_k = &shares[0..k];
    println!("Attempting reconstruction with {} shares...", k);
    let reconstructed_k = reconstruct_secret(subset_k);
    println!("  Reconstructed Secret: {}", reconstructed_k);
    println!("  Success? {}\n", reconstructed_k == secret);

    // 4. Reconstruct with all N shares
    println!("Attempting reconstruction with all {} shares...", n);
    let reconstructed_n = reconstruct_secret(&shares);
    println!("  Reconstructed Secret: {}", reconstructed_n);
    println!("  Success? {}\n", reconstructed_n == secret);

    // 5. Demonstrate failure with K-1 shares
    let subset_k_minus_1 = &shares[0..k-1];
    println!("Attempting reconstruction with only {} shares (below threshold)...", k - 1);
    let reconstructed_fail = reconstruct_secret(subset_k_minus_1);
    println!("  Reconstructed result: {}", reconstructed_fail);
    println!("  Incorrect as expected? {}\n", reconstructed_fail != secret);

    println!("--- Demo Completed ---");
}
