use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty: i32 = item_quantity.parse()?;

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("3"), Ok(16));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert!(total_cost("beep boop").is_err());
    }
}
