use std::fmt::Display;

#[derive(Debug)]
enum EntropyError {
    OutOfRange { p: f32 },
    SumNotOne { sum: f32 },
}

impl std::error::Error for EntropyError {}
impl Display for EntropyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntropyError::OutOfRange { p } => {
                write!(f, "probability out of range: {p} (expected 0.0..=1.0)")
            }
            EntropyError::SumNotOne { sum } => {
                write!(f, "probabilities must sum to 1.0, got {sum}")
            }
        }
    }
}

fn term(p: f32) -> f32 {
    if p == 0.0 { 0.0 } else { -p * p.log2() }
}

fn entropy(phead: f32, ptail: f32) -> Result<f32, EntropyError> {
    if phead < 0.0 || phead > 1.0 {
        return Err(EntropyError::OutOfRange { p: phead });
    }

    if ptail < 0.0 || ptail > 1.0 {
        return Err(EntropyError::OutOfRange { p: ptail });
    }

    let sum = ptail + phead;
    if f32::abs(sum - 1.0_f32) > 1e-6 {
        return Err(EntropyError::SumNotOne { sum });
    }

    let entropy = term(phead) + term(ptail);
    Ok(entropy)
}

#[cfg(test)]
mod tests {
    use crate::entropy::entropy;

    #[test]
    fn compute_entropy() {
        let pheads = vec![0.50, 0.75, 0.80, 0.10];
        let ptails = vec![0.50, 0.25, 0.20, 0.90];
        let expected = vec![1.00, 0.81, 0.72, 0.47];

        for i in 0..pheads.len() {
            let entropy = entropy(pheads[i], ptails[i]).unwrap();
            assert!((entropy - expected[i]).abs() < 1e-2);
        }
    }
}
