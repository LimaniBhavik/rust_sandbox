// fn main() {
//     let tuple:(i32,f64,u8) = (-325,4.9,22);
//     println!("{:?}",tuple);
//  }

// fn main() {
//     let tuple:(i32,f64,u8) = (-325,4.9,22);
//     println!("integer is :{:?}",tuple.0);
//     println!("float is :{:?}",tuple.1);
//     println!("unsigned integer is :{:?}",tuple.2);
//  }

// fn main(){
//     let b:(i32,bool,f64) = (110,true,10.9);
//     print(b);
//  }
//  //pass the tuple as a parameter
 
//  fn print(x:(i32,bool,f64)){
//     println!("Inside print method");
//     println!("{:?}",x);
//  }

fn main(){
    let b:(i32,bool,f64) = (30,true,7.9);
    print(b);
 }
 fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    let (age,is_male,cgpa) = x; //assigns a tuple to distinct variables
    println!("Age is {} , isMale? {},cgpa is {}",age,is_male,cgpa);
 }