pub fn sum_of_even_numbers(input: &[i32]) -> i32 {
    input.iter().filter(|&&x| x % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_even_numbers() {
        assert_eq!(sum_of_even_numbers(&[1, 2, 3, 4, 5]), 6);
        assert_eq!(sum_of_even_numbers(&[0, 6, 7, 8, 9]), 14);
    }
}
