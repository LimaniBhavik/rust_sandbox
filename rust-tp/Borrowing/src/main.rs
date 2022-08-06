// fn main(){
//     // a list of nos
//     let v = vec![10,20,30];
//     print_vector(&v); // passing reference
//     println!("Printing the value from main() v[0]={}",v[0]);
//  }
//  fn print_vector(x:&Vec<i32>){
//     println!("Inside print_vector function {:?}",x);
//  }

// fn add_one(e: &mut i32) {
//     *e+= 1;
//  }
//  fn main() {
//     let mut i = 3;
//     add_one(&mut i);
//     println!("{}", i);
//  }

fn main() {
    let mut name:String = String::from("TutorialsPoint");
    display(&mut name); 
    //pass a mutable reference of name
    println!("The value of name after modification is:{}",name);
 }
 fn display(param_name:&mut String){
    println!("param_name value is :{}",param_name);
    param_name.push_str(" Rocks"); 
    //Modify the actual string,name
 }