use std::str::FromStr;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(());
        }
        Ok(Color {
            red: parts[0].parse().map_err(|_| ())?,
            green: parts[1].parse().map_err(|_| ())?,
            blue: parts[2].parse().map_err(|_| ())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let c = "128,0,255".parse::<Color>();
        assert!(c.is_ok());
        let c = c.unwrap();
        assert_eq!(c.red, 128);
        assert_eq!(c.green, 0);
        assert_eq!(c.blue, 255);
    }

    #[test]
    fn test_invalid_color() {
        let c = "128,invalid,255".parse::<Color>();
        assert!(c.is_err());
    }
}
