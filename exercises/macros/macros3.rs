mod macros {
    #[macro_export] // Use this attribute to make the macro available outside the module
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!(); // Now the macro can be called
}
