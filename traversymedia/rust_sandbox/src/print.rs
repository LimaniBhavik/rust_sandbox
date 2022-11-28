pub fn run(){
    // Print to the Console
    println!("Hello from the print.rs file!");

    // Basic Formatting
    println!("Number: {}",11);
    println!("{} is from the {}!","Bhavik", "India");

    // Positional Arguments
    println!("{0} is from {1} and {0} City is {2}.", "Bhavik", "India", "Ahmedabad");

    // Named Arguments"
    println!("{name} love to wirte the {hobby}!",name="Bhavik", hobby="Web3 Code");

    // Placeholder traits 
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 255, 255, 255);

    // Placeholder for debug trait
    println!("{:?}", (122, false, "hello web3"));

    // Basic Math
    println!("102 + 102 = {}", 102+102);
}