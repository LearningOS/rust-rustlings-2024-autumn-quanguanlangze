use std::borrow::Cow;

fn main() {
    let hello = "Hello";
    let hello_cow: Cow<str> = Cow::Borrowed(hello);

    let world = "world".to_string();
    let world_cow: Cow<str> = Cow::Owned(world);

    println!("{}, {}", hello_cow, world_cow);
}
