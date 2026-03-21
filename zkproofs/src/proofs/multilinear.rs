use ark_ff::PrimeField;

#[derive(Clone)]
pub(crate) struct MultilinearPoly<F: PrimeField> {
    pub(crate) evals: Vec<F>,
    pub(crate) n_vars: usize,
}

impl<F: PrimeField> MultilinearPoly<F> {
    pub(crate) fn new(n_vars: usize, evaluations: Vec<F>) -> Self {
        if evaluations.len() != 1 << n_vars {
            panic!("what are you doing?");
        }

        Self {
            evals: evaluations,
            n_vars,
        }
    }

    pub(crate) fn evaluate(&self, assignments: &[F]) -> F {
        if assignments.len() != self.n_vars {
            panic!("what are you doing again?");
        }

        let mut poly = self.clone();

        for val in assignments {
            poly = poly.partial_evaluate(0, val);
        }

        poly.evals[0]
    }

    pub(crate) fn partial_evaluate(&self, index: usize, value: &F) -> Self {
    
        let mut result = vec![];
        for (a, b) in pairs(index, self.n_vars).into_iter() {
            let a = self.evals[a];
            let b = self.evals[b];
            result.push(a + *value * (b - a));
        }

        Self::new(self.n_vars - 1, result)
    }
}


fn pairs(index: usize, n_vars: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let target_hc = n_vars - 1;
    for val in 0..(1 << target_hc) {
        let inverted_index = n_vars - index - 1;
        let insert_zero = insert_bit(val, inverted_index);
        let insert_one = insert_zero | (1 << inverted_index);
        result.push((insert_zero, insert_one));
    }
    result
}


fn insert_bit(value: usize, index: usize) -> usize {
   
    let high = value >> index;
    let mask = (1 << index) - 1;
    let low = value & mask;

    high << index + 1 | low
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::multilinear::{insert_bit, pairs, MultilinearPoly};
    use ark_bn254::Fr;

    pub(crate) fn to_field(input: Vec<u64>) -> Vec<Fr> {
        input.into_iter().map(|v| Fr::from(v)).collect()
    }

    #[test]
    fn bit_insertion() {
        assert_eq!(insert_bit(3, 0), 0b110);
        assert_eq!(insert_bit(3, 1), 0b101);
        assert_eq!(insert_bit(3, 2), 0b011);
    }

    #[test]
    fn test_pairs() {

        let pairs = pairs(2, 3);
    }

    #[test]
    fn test_partial_evaluate() {
        // 2ab + 3bc
        let poly = MultilinearPoly::new(3, to_field(vec![0_u64, 0, 0, 3, 0, 0, 2, 5]));
        assert_eq!(
            poly.partial_evaluate(2, &Fr::from(3)).evals,
            to_field(vec![0, 9, 0, 11])
        );
        assert_eq!(
            poly.partial_evaluate(1, &Fr::from(3)).evals,
            to_field(vec![0, 9, 6, 15])
        );
    }
}