use std::f32;

fn main() {
    let pi = f32::consts::PI; // Use the standard constant for pi
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
