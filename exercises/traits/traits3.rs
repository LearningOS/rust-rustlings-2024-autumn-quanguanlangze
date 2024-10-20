trait Licensed {
    fn licensing_info(&self) -> String;
}

impl Licensed for SomeSoftware {
    fn licensing_info(&self) -> String {
        String::from("Licensed under MIT")
    }
}

impl Licensed for OtherSoftware {
    fn licensing_info(&self) -> String {
        String::from("Licensed under MIT")
    }
}

struct SomeSoftware;
struct OtherSoftware;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_licenses() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert_eq!(some_software.licensing_info(), other_software.licensing_info());
    }
}
