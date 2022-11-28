// fn main() {
//     let n1 = "Tutorials".to_string();
//     println!("length of string is {}",n1.len());
//     let c1 = &n1[4..9]; 
    
//     // fetches characters at 4,5,6,7, and 8 indexes
//     println!("{}",c1);
//  }

// fn main(){
//     let data = [10,20,30,40,50];
//     use_slice(&data[1..4]);
//     //this is effectively borrowing elements for a while
//  }
//  fn use_slice(slice:&[i32]) { 
//     // is taking a slice or borrowing a part of an array of i32s
//     println!("length of slice is {:?}",slice.len());
//     println!("{:?}",slice);
//  }

fn main(){
    let mut data = [10,20,30,40,50];
    use_slice(&mut data[1..4]);
    // passes references of 20, 30 and 40
    println!("{:?}",data);
 }
 fn use_slice(slice:&mut [i32]) {
    println!("length of slice is {:?}",slice.len());
    println!("{:?}",slice);
    slice[0] = 1010; // replaces 20 with 1010
 }