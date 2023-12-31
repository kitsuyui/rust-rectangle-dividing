use num_traits::{Num, NumAssignOps, NumOps};
// weights are just Vec<T>

pub(crate) fn normalize_weights<T>(weights: &[T]) -> Vec<T>
where
    T: Copy + Num + NumAssignOps + NumOps + for<'a> std::iter::Sum<&'a T>,
{
    let sum: T = weights.iter().sum();
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
}
