// fn main() {
//     let mut v = Vec::new();
//     v.push(20);
//     v.push(30);
//     v.push(40);
 
//     println!("size of vector is :{}",v.len());
//     println!("{:?}",v);
//  }

// fn main() {
//     let v = vec![1,2,3];
//     println!("{:?}",v);
//  }

// fn main() {
//     let mut v = vec![10,20,30];
//     v.remove(1);
//     println!("{:?}",v);
//  }

// fn main() {
//     let v = vec![10,20,30];
//     if v.contains(&10) {
//        println!("found 10");
//     }
//     println!("{:?}",v);
//  }
 
// fn main() {
//     let v = vec![1,2,3];
//     println!("size of vector is :{}",v.len());
//  }

// fn main() {
//     let mut v = Vec::new();
//     v.push(20);
//     v.push(30);
 
//     println!("{:?}",v[1]);
//  }
 
// fn main() {
//     let mut v = Vec::new();
//     v.push(20);
//     v.push(30);
//     v.push(40);
//     v.push(500);
 
//     for i in &v {
//        println!("{}",i);
//     }
//     println!("{:?}",v);
//  }

// use std::collections::HashMap;
// fn main(){
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    println!("{:?}",stateCodes);
// }

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    println!("size of map is {}",stateCodes.len());
// }

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    println!("size of map is {}",stateCodes.len());
//    println!("{:?}",stateCodes);

//    match stateCodes.get(&"KL") {
//       Some(value)=> {
//          println!("Value for key KL is {}",value);
//       }
//       None => {
//          println!("nothing found");
//       }
//    }
// }

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("GJ","Gujarat");
//    stateCodes.insert("MH","Maharashtra");

//    for (key, val) in stateCodes.iter() {
//       println!("key: {} val: {}", key, val);
//    }
// }

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    stateCodes.insert("GJ","Gujarat");

//    if stateCodes.contains_key(&"GJ") {
//       println!("found key");
//    }
// }

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    stateCodes.insert("GJ","Gujarat");

//    println!("length of the hashmap {}",stateCodes.len());
//    stateCodes.remove(&"MH");
//    println!("length of the hashmap after remove() {}",stateCodes.len());
//    println!("{:?}",stateCodes);
// }

// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();

//    names.insert("TP02");
//    names.insert("Kannan");
//    names.insert("TP02");//duplicates not added
//    names.insert("Mohtashim");

//    println!("{:?}",names);
// }

// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();
//    names.insert("Mohtashim");
//    names.insert("Kannan");
//    names.insert("TutorialsPoint");
//    println!("size of the set is {}",names.len());
// }

// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();
//    names.insert("Mohtashim");
//    names.insert("Kannan");
//    names.insert("TutorialsPoint");
//    names.insert("Mohtashim");

//    for name in names.iter() {
//       println!("{}",name);
//    }
// }

// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();
//    names.insert("Mohtashim");
//    names.insert("Kannan");
//    names.insert("TutorialsPoint");
//    names.insert("Mohtashim");

//    match names.get(&"Mohtashim"){
//       Some(value)=>{
//          println!("found {}",value);
//       }
//       None =>{
//          println!("not found");
//       }
//    }
//    println!("{:?}",names);
// }

// use std::collections::HashSet;

// fn main() {
//    let mut names = HashSet::new();
//    names.insert("Mohtashim");
//    names.insert("Kannan");
//    names.insert("TutorialsPoint");

//    if names.contains(&"Kannan") {
//       println!("found name");
//    }  
// }

use std::collections::HashSet;

fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   println!("length of the Hashset: {}",names.len());
   names.remove(&"Kannan");
   println!("length of the Hashset after remove() : {}",names.len());
}