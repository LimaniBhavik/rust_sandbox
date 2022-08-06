// fn main() {
//     let company:&str="TutorialsPoint";
//     let location:&str = "Hyderabad";
//     println!("company is : {} location :{}",company,location);
//  }

// fn main() {
//     let company:&'static str = "TutorialsPoint";
//     let location:&'static str = "Hyderabad";
//     println!("company is : {} location :{}",company,location);
//  }

// fn main(){
//     let empty_string = String::new();
//     println!("length is {}",empty_string.len());
 
//     let content_string = String::from("TutorialsPoint");
//     println!("length is {}",content_string.len());
//  }

fn main(){
    let mut z = String::new();
    z.push_str("hello");
  
    println!("{}",z);
 }