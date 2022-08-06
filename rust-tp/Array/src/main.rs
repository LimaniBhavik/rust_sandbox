// fn main(){
//     let arr:[i32;4] = [10,20,30,40];
//     println!("array is {:?}",arr);
//     println!("array size is :{}",arr.len());
//  }

// fn main(){
//     let arr = [10,20,30,40];
//     println!("array is {:?}",arr);
//     println!("array size is :{}",arr.len());
//  }

// fn main() {
//     let arr:[i32;4] = [-1;4];
//     println!("array is {:?}",arr);
//     println!("array size is :{}",arr.len());
//  }

// fn main(){
//     let arr:[i32;4] = [10,20,30,40];
//     println!("array is {:?}",arr);
//     println!("array size is :{}",arr.len());
//     let i = 0;
//     let arraylen = arr.len();

//     for index in i..arraylen {
//        println!("index is: {} & value is : {}",index,arr[index]);
//     }
//  }

// fn main(){

//     let arr:[i32;4] = [10,20,30,40];
//        println!("array is {:?}",arr);
//        println!("array size is :{}",arr.len());
    
//        for val in arr.iter(){
//           println!("value is :{}",val);
//        }
//     }

// fn main(){
//     let mut arr:[i32;4] = [10,20,30,40];
//     arr[1] = 0;
//     println!("{:?}",arr);
//  }

// fn main() {
//     let arr = [10,20,30];
//     update(arr);
 
//     print!("Inside main {:?}",arr);
//  }
//  fn update(mut arr:[i32;3]){
//     for i in 0..arr.len() {
//        arr[i] = 0;
//     }
//     println!("Inside update {:?}",arr);
//  }

// fn main() {
//     let mut arr = [10,20,30];
//     update(&mut arr);
//     print!("Inside main {:?}",arr);
//  }
//  fn update(arr:&mut [i32;3]){
//     for i in 0..3 {
//        arr[i] = 0;
//     }
//     println!("Inside update {:?}",arr);
//  }
fn main() {
    const N: usize = 20; 
    // pointer sized
    let arr = [0; N];
 
    print!(" {}",arr[10]);
 }