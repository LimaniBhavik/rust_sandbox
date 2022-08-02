pub fn run(){
    let name = "Bhavik";
    let mut age=27;
    
    println!("My Name is {} and I'm {}", name, age);
    age = 26;
    println!("My Name is {} and I'm {}", name, age);

    // Define Cont
    const ID: i32= 007;
    println!("Your ID is:{}",ID);

    // Multiple values in Var
    let (my_name,my_age) = ("Bhavik", 27);
    println!("{} is {}",my_name,my_age);

}