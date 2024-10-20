use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl TryFrom<&[i16]> for Color {
    type Error = String;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err("Invalid slice length".into());
        }
        let red = u8::try_from(slice[0]).map_err(|_| "Invalid red value")?;
        let green = u8::try_from(slice[1]).map_err(|_| "Invalid green value")?;
        let blue = u8::try_from(slice[2]).map_err(|_| "Invalid blue value")?;
        Ok(Color { red, green, blue })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_try_from_slice() {
        let color = Color::try_from(&[128, 255, 0][..]);
        assert!(color.is_ok());
        let color = color.unwrap();
        assert_eq!(color, Color { red: 128, green: 255, blue: 0 });
    }

    #[test]
    fn test_invalid_slice() {
        let color = Color::try_from(&[256, -1, 0][..]);
        assert!(color.is_err());
    }
}
