use rand::Rng;

pub const PRIME: i128 = 2_305_843_009_213_693_951;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Share {
    pub x: i128,
    pub y: i128,
}

fn add(a: i128, b: i128) -> i128 {
    (a + b).rem_euclid(PRIME)
}

fn sub(a: i128, b: i128) -> i128 {
    (a - b).rem_euclid(PRIME)
}

fn mul(a: i128, b: i128) -> i128 {
    ((a as i128 * b as i128)).rem_euclid(PRIME)
}


fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inverse(a: i128) -> i128 {
    let (g, x, _) = extended_gcd(a, PRIME);
    if g != 1 {
        panic!("Modular inverse does not exist");
    }
    x.rem_euclid(PRIME)
}

fn div(a: i128, b: i128) -> i128 {
    mul(a, mod_inverse(b))
}

fn evaluate_polynomial(coeffs: &[i128], x: i128) -> i128 {
    let mut result = 0;
    for &coeff in coeffs.iter().rev() {
        result = add(mul(result, x), coeff);
    }
    result
}

pub fn split_secret(secret: i128, n: usize, k: usize) -> Vec<Share> {
    assert!(k <= n, "Threshold K cannot be greater than total shares N");
    assert!(secret < PRIME, "Secret must be smaller than the field prime");

    let mut rng = rand::thread_rng();
    let mut coeffs = vec![secret]; 

    for _ in 1..k {
        let rand_coeff = rng.gen_range(0..PRIME);
        coeffs.push(rand_coeff);
    }

    (1..=n)
        .map(|x| Share {
            x: x as i128,
            y: evaluate_polynomial(&coeffs, x as i128),
        })
        .collect()
}


pub fn reconstruct_secret(shares: &[Share]) -> i128 {
    let mut secret = 0;

    for (j, share_j) in shares.iter().enumerate() {
        let mut basis = 1;

        for (m, share_m) in shares.iter().enumerate() {
            if j == m {
                continue;
            }

            let num = sub(0, share_m.x);
            let den = sub(share_j.x, share_m.x);
            
            basis = mul(basis, div(num, den));
        }

        secret = add(secret, mul(share_j.y, basis));
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_reconstruction() {
        let secret = 123456789;
        let n = 5;
        let k = 3;
        let shares = split_secret(secret, n, k);
        
        let reconstructed = reconstruct_secret(&shares[0..k]);
        assert_eq!(reconstructed, secret);
    }

    #[test]
    fn test_different_subsets() {
        let secret = 987654321;
        let n = 10;
        let k = 4;
        let shares = split_secret(secret, n, k);
        
        assert_eq!(reconstruct_secret(&shares[0..4]), secret);
        
        assert_eq!(reconstruct_secret(&shares[6..10]), secret);
        
        let odd_shares = shares.iter().step_by(2).cloned().collect::<Vec<_>>();
        assert_eq!(reconstruct_secret(&odd_shares[0..4]), secret);
    }

    #[test]
    #[should_panic]
    fn test_secret_too_large() {
        let secret = PRIME + 1;
        split_secret(secret, 5, 3);
    }

    #[test]
    fn test_insufficient_shares() {
        let secret = 42;
        let n = 5;
        let k = 3;
        let shares = split_secret(secret, n, k);
        
        
        let reconstructed = reconstruct_secret(&shares[0..k-1]);
        assert_ne!(reconstructed, secret);
    }
}
