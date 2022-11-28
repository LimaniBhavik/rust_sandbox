// fn main(){
//     for x in 1..11{ // 11 is not inclusive
//        if x==5 {
//           continue;
//        }
//        println!("x is {}",x);
//     }
//  }

// fn main(){
//    let mut x = 0;
//    while x < 10{
//       x+=1;
//       println!("inside loop x value is {}",x);
//    }
//    println!("outside loop x value is {}",x);
// }


// fn main(){
//    //while true

//    let mut x = 0;
//    loop {
//       x+=1;
//       println!("x={}",x);

//       if x==15 {
//          break;
//       }
//    }
// }

fn main() {

   let mut count = 0;

   for num in 0..21 {
      if num % 2==0 {
         continue;
      }
      count+=1;
   }
   println! (" The count of odd values between 0 and 20 is: {} ",count);
   //outputs 10
}