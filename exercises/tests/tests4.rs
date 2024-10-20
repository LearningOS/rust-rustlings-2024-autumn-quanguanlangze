struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative or zero!");
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_negative_width() {
        Rectangle::new(-1, 10); 
    }

    #[test]
    #[should_panic]
    fn test_zero_height() {
        Rectangle::new(10, 0); 
    }

    #[test]
    fn test_valid_dimensions() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }
}
