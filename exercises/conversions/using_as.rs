fn average(values: &[f64]) -> f64 {
    let total: f64 = values.iter().sum();
    total / values.len() as f64 // Cast len() to f64 to match types
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
