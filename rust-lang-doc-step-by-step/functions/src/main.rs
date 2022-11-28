// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn main() {
//     print_labeled_measurement(55, 'h', 0.0, "The measurement is:");
// }

// fn print_labeled_measurement(value: i32, unit_label: char, _flt_value: f32, string_values: &str) {
//     println!("{string_values}{value}{unit_label}{_flt_value}");
// }

// fn main() {
//     let y = { let x = 3; x + 1 };

//     println!("The value of y is: {y}");
// }

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

fn main() {
    let x = plus_one(5+9);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 + 9
}