// fn main() {
//     let mut x = 5; //Variable Mutability
//  &   println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32

//     println!("Values : {} {}", x, y);
// }

// fn main() {
//     // addition
//     let sum = 5 + 10;
//     println!("Addition : {}", sum);

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("Subtraction : {}", difference);

//     // multiplication
//     let product = 4 * 30;
//     println!("Multiplication : {}", product);

//     // division
//     let quotient = 56.7 / 32.2;
//     println!("Division by f64 : {}", quotient);
//     let floored = 2 / 3; // Results in 0
//     println!("Division by i32 : {}", floored);

//     // remainder
//     let remainder = 43 % 5;
//     println!("Remainder: {}", remainder);
// }

fn main() {
    let t = true;
    println!("Bool: {}", t);
    let f: bool = false; // with explicit type annotation
    println!("Bool: {}", f);
}

