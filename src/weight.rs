use num_traits::{Num, NumAssignOps, NumOps};
// weights are just Vec<T>

pub(crate) fn normalize_weights<T>(weights: &[T]) -> Vec<T>
where
    T: Copy + Num + NumAssignOps + NumOps + for<'a> std::iter::Sum<&'a T>,
{
    if weights.is_empty() {
        return vec![];
    }

    let sum: T = weights.iter().sum();
    if sum == T::zero() {
        let count = weights.iter().fold(T::zero(), |acc, _| acc + T::one());
        return weights.iter().map(|_| T::one() / count).collect();
    }

    weights.iter().map(|w| *w / sum).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_weights() {
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let normalized = normalize_weights(&weights);
        assert_eq!(normalized, vec![0.25, 0.25, 0.25, 0.25]);

        let weights = vec![1.0, 2.0, 3.0, 4.0];
        let normalized = normalize_weights(&weights);
        assert_eq!(normalized, vec![0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn test_normalize_zero_sum_weights() {
        let weights = vec![0.0, 0.0, 0.0, 0.0];
        let normalized = normalize_weights(&weights);
        assert_eq!(normalized, vec![0.25, 0.25, 0.25, 0.25]);

        let weights = vec![-1.0, 1.0];
        let normalized = normalize_weights(&weights);
        assert_eq!(normalized, vec![0.5, 0.5]);
    }
}
