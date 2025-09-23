// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    let _ = my_option; // Remove unwrap after is_none check, simply use the variable to avoid unused warning

    let my_arr = &[-1, -2, -7, -5, -6]; // Simplify the expression -3 - 4 to -7
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // Use clear() instead of resize to empty the vector
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b); // Use std::mem::swap for idiomatic swapping
    println!("value a: {}; value b: {}", value_a, value_b);
}
