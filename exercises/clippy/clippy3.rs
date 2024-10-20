#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // Do not unwrap an option if it's already checked for None
    if my_option.is_none() {
        // Do nothing
    }

    let my_arr = &[
        -1, -2, -3, // Missing commas
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![]; // Correct way to declare an empty vector
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Correct swap using std::mem::swap
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
