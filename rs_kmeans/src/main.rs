mod k_means;
mod solver;

type Point = Vec<f64>;

fn mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let numbers = vec![1.0, 2.0, 3.0];
        assert_eq!(mean(&numbers), 2.0);
    }

    #[test]
    fn test_distance() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert_eq!(distance(&a, &b), 5.196152422706632);
    }

    #[test]
    fn test_distance_zero() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![0.0, 0.0, 0.0];
        assert_eq!(distance(&a, &b), 0.0);
    }

    #[test]
    fn test_distance_negative() {
        let a = vec![-1.0, -2.0, -3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert_eq!(distance(&a, &b), 12.449899597988733);
    }
}
