// // Stack
// // Heap
// fn main(){
//     let v = vec![1,2,3]; 
//     // vector v owns the object in heap
 
//     //only a single variable owns the heap memory at any given time
//     let v2 = v; 
//     // here two variables owns heap value,
//     //two pointers to the same content is not allowed in rust
 
//     //Rust is very smart in terms of memory access ,so it detects a race condition
//     //as two variables point to same heap
 
//     println!("{:?}",v);
//  }

// fn main(){
//     let v = vec![1,2,3];     // vector v owns the object in heap
//     let v2 = v;              // moves ownership to v2
//     display(v2);             // v2 is moved to display and v2 is invalidated
//     println!("In main {:?}",v2);    //v2 is No longer usable here
//  }
//  fn display(v:Vec<i32>){
//     println!("inside display {:?}",v);
//  }

// fn main(){
//     let v = vec![1,2,3];       // vector v owns the object in heap
//     let v2 = v;                // moves ownership to v2
//     let v2_return = display(v2);    
//     println!("In main {:?}",v2_return);
//  }
//  fn display(v:Vec<i32>)->Vec<i32> { 
//     // returning same vector
//     println!("inside display {:?}",v);
//  }

fn main(){
    let u1 = 10;
    let u2 = u1;  // u1 value copied(not moved) to u2
 
    println!("u1 = {}",u1);
 }