// // The `derive` attribute automatically creates the implementation
// // required to make this `enum` printable with `fmt::Debug`.
// #[derive(Debug)]
// enum GenderCategory {
// Male,Female
// }
// fn main() {
// let male = GenderCategory::Male;
// let female = GenderCategory::Female;

// println!("{:?}",male);
// println!("{:?}",female);
// }

// // The `derive` attribute automatically creates the implementation
// // required to make this `enum` printable with `fmt::Debug`.

// #[derive(Debug)]
// enum GenderCategory {
//    Male,Female
// }

// // The `derive` attribute automatically creates the implementation
// // required to make this `struct` printable with `fmt::Debug`.
// #[derive(Debug)]
// struct Person {
//    name:String,
//    gender:GenderCategory
// }

// fn main() {
//    let p1 = Person {
//       name:String::from("Mohtashim"),
//       gender:GenderCategory::Male
//    };
//    let p2 = Person {
//       name:String::from("Amy"),
//       gender:GenderCategory::Female
//    };
//    println!("{:?}",p1);
//    println!("{:?}",p2);
// }


// fn main() {
//     let result = is_even(3);
//     println!("{:?}",result);
//     println!("{:?}",is_even(30));
//  }
//  fn is_even(no:i32)->Option<bool> {
//     if no%2 == 0 {
//        Some(true)
//     } else {
//        None
//     }
//  }

// enum CarType {
//     Hatch,
//     Sedan,
//     SUV
//  }
//  fn print_size(car:CarType) {
//     match car {
//        CarType::Hatch => {
//           println!("Small sized car");
//        },
//        CarType::Sedan => {
//           println!("medium sized car");
//        },
//        CarType::SUV =>{
//           println!("Large sized Sports Utility car");
//        }
//     }
//  }
//  fn main(){
//     print_size(CarType::SUV);
//     print_size(CarType::Hatch);
//     print_size(CarType::Sedan);
//  }

// fn main() {
//     match is_even(5) {
//        Some(data) => {
//           if data==true {
//              println!("Even no");
//           }
//        },
//        None => {
//           println!("not even");
//        }
//     }
//  }
//  fn is_even(no:i32)->Option<bool> {
//     if no%2 == 0 {
//        Some(true)
//     } else {
//        None
//     }
//  }

// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory {
   Name(String),Usr_ID(i32)
}
fn main() {
   let p1 = GenderCategory::Name(String::from("Bhavik"));
   let p2 = GenderCategory::Usr_ID(0072);
   println!("{:?}",p1);
   println!("{:?}",p2);

   match p1 {
      GenderCategory::Name(val)=> {
         println!("{}",val);
      }
      GenderCategory::Usr_ID(val)=> {
         println!("{}",val);
      }
   }
}
