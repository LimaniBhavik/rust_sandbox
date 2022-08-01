// fn main() {
//     let mut x = 5; //Variable Mutability
//  &&&&&   println!("The value of x is: {x}");
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

// fn main() {
//     let t = true;
//     println!("Bool: {}", t);
//     let f: bool = false; // with explicit type annotation
//     println!("Bool: {}", f);
// }

// fn main() {
//     let c = 'z';
//     println!("The Character Type: {}", c);
//     let z: char = 'ℤ'; // with explicit type annotation
//     println!("The Character Type: {}", z);
//     let heart_eyed_cat = '😻';
//     println!("The Character Type: {}", heart_eyed_cat);
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     println!("Tuple of i32 f64 u8 {:?}",tup);
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;
//     println!("Assign and get value {}  Direct get value {}", five_hundred, x.0);
//     let six_point_four = x.1;
//     println!("The value of y is: {}",six_point_four);
//     let one = x.2;
//     println!("The value of y is: {}",one);
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     print!("{:?}",a);
    
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];
//     print!("{:?}",months);

//     let array: [i32; 5] = [1, 2, 3, 4, 5];
//     print!("{:?}",array);

//     let a = [3; 5];
//     print!("{:?}",a);

//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     print!("{:?}",first);
//     let second = a[1];
//     print!("{:?}",second);
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
