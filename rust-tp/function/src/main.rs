// fn main(){
//     //calling a function
//     fn_hello();
//  }
//  //Defining a function
//  fn fn_hello(){
//     println!("hello from function fn_hello ");
//  }

// fn main(){
//     println!("pi value is {}",get_pi());
//  }

//  fn get_pi()->f64 {
//     22.0/7.0
//  }

// fn main(){
//     let no:i32 = 5;
//     mutate_no_to_zero(no);
//     println!("The value of no is:{}",no);
//  }
 
//  fn mutate_no_to_zero(mut param_no: i32) {
//     param_no = param_no*0;
//     println!("param_no value is :{}",param_no);
//  }

// fn main() {
//     let mut no:i32 = 5;
//     mutate_no_to_zero(&mut no);
//     println!("The value of no is:{}",no);
//  }
//  fn mutate_no_to_zero(param_no:&mut i32){
//     *param_no = 0; //de reference
//  }

fn main(){
    let name:String = String::from("BhavikLimani");
    display(name); 
    //cannot access name after display
 }
 fn display(param_name:String){
    println!("param_name value is :{}",param_name);
 }
 