extern crate movies_lib;

use movies_lib::movies::play;

fn main() {
    println!("inside main of test ");
    play("Yeahhh this run from the module!".to_string())
}