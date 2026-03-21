use ark_ff::PrimeField;
use crate::multilinear::MultilinearPoly;

pub struct SumcheckProof<F: PrimeField> {

    pub round_polys: Vec<[F; 2]>,
}

pub struct Prover<F: PrimeField> {
    poly: MultilinearPoly<F>,
    pub claimed_sum: F,
}

impl<F: PrimeField> Prover<F> {
    pub fn new(poly: MultilinearPoly<F>) -> Self {
        let claimed_sum = poly.evals.iter().copied().sum();
        Self { poly, claimed_sum }
    }

    fn compute_round_poly(&self) -> [F; 2] {
        let at_0: F = self.poly
            .partial_evaluate(0, &F::zero())
            .evals
            .iter()
            .copied()
            .sum();

        let at_1: F = self.poly
            .partial_evaluate(0, &F::one())
            .evals
            .iter()
            .copied()
            .sum();

        [at_0, at_1]
    }

    pub fn prove(&mut self, challenges: &[F]) -> SumcheckProof<F> {
        assert_eq!(challenges.len(), self.poly.n_vars);
        let mut round_polys = vec![];

        for &r in challenges {
            let round_poly = self.compute_round_poly();
            round_polys.push(round_poly);
            // Fix variable 0 to the challenge, reduce by one var
            self.poly = self.poly.partial_evaluate(0, &r);
        }

        SumcheckProof { round_polys }
    }
}

pub struct Verifier;

impl Verifier {
    pub fn verify<F: PrimeField>(
        proof: &SumcheckProof<F>,
        claimed_sum: F,
        challenges: &[F],
        final_eval: F,
    ) -> bool {
        let mut expected = claimed_sum;

        for ([at_0, at_1], &r) in proof.round_polys.iter().zip(challenges.iter()) {
            // Check s_i(0) + s_i(1) == value from previous round
            if *at_0 + *at_1 != expected {
                println!("round check failed: {} + {} != {}", at_0, at_1, expected);
                return false;
            }
            // Evaluate s_i at the challenge point using linear interpolation
            // s_i(r) = at_0 + r * (at_1 - at_0)
            expected = *at_0 + r * (*at_1 - *at_0);
        }

        // Last expected == oracle eval of original poly at all challenges
        if expected != final_eval {
            println!("final check failed: {} != {}", expected, final_eval);
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multilinear::{tests::to_field, MultilinearPoly};
    use ark_bn254::Fr;

    #[test]
    fn test_sumcheck() {
        // f(a,b,c) = 2ab + 3bc
        // sum over {0,1}^3 = 0+0+0+3+0+0+2+5 = 10
        let poly = MultilinearPoly::new(3, to_field(vec![0, 0, 0, 3, 0, 0, 2, 5]));

        let mut prover = Prover::new(poly.clone());
        assert_eq!(prover.claimed_sum, Fr::from(10u64));

 
        let challenges = vec![Fr::from(2u64), Fr::from(3u64), Fr::from(4u64)];

        let proof = prover.prove(&challenges);

        // Verifier computes final eval themselves (oracle)
        let final_eval = poly.evaluate(&challenges);

        let valid = Verifier::verify(&proof, Fr::from(10u64), &challenges, final_eval);
        assert!(valid);
    }

    #[test]
    fn test_round_values() {
        // Manually verify round 1 values match our hand calculation
        // s1(0) = 3, s1(1) = 7
        let poly = MultilinearPoly::new(3, to_field(vec![0, 0, 0, 3, 0, 0, 2, 5]));
        let mut prover = Prover::new(poly);

        let round_1 = prover.compute_round_poly();
        assert_eq!(round_1[0], Fr::from(3u64)); // s1(0)
        assert_eq!(round_1[1], Fr::from(7u64)); // s1(1)
    }
}
```

---

## The flow as a diagram
```
Prover                              Verifier
------                              --------
H = sum of all evals        →       check H is plausible

compute s1(0), s1(1)        →       check s1(0)+s1(1) == H
                            ←       send r1 (random)

fix a=r1, compute s2(0),s2(1) →    check s2(0)+s2(1) == s1(r1)
                            ←       send r2

fix b=r2, compute s3(0),s3(1) →    check s3(0)+s3(1) == s2(r2)
                            ←       send r3

                                    check s3(r3) == f(r1,r2,r3)  ← oracle