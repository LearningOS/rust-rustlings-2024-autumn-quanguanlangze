use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Mango,
    Lychee,
    Banana, // Adding new fruit types
    Orange,
}

fn fruit_basket() -> HashMap<Fruit, u32> {
    let mut basket = HashMap::new();
    
    // Initial fruits
    basket.insert(Fruit::Apple, 4);
    basket.insert(Fruit::Mango, 2);
    basket.insert(Fruit::Lychee, 5);
    
    // Add more fruits to ensure the total count is more than 11
    basket.insert(Fruit::Banana, 3);
    basket.insert(Fruit::Orange, 2);
    
    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_eleven_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() > 11);
    }
}
