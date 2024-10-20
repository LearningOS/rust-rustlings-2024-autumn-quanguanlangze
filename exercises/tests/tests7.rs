extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"] // Alias for my_demo_function
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    #[no_mangle] // Prevents symbol mangling for my_demo_function
    pub extern "Rust" fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // SAFETY: We know those functions are aliases of a safe Rust function.
        unsafe {
            assert_eq!(my_demo_function(123), 123);
            assert_eq!(my_demo_function_alias(456), 456); // Alias of my_demo_function
        }
    }
}
