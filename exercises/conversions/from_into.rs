struct Person {
    name: String,
    age: u8,
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let parts: Vec<&str> = s.split(',').collect();
        Person {
            name: parts[0].to_string(),
            age: parts[1].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let p = Person::from("John,32");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
}
