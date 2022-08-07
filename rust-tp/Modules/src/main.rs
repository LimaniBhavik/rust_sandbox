// pub mod movies {
//     pub fn play(name:String) {
//        println!("Playing movie {}",name);
//     }
//  }
//  fn main(){
//     movies::play("Herold and Kumar".to_string());
//  }

// pub mod movies {
//     pub fn play(name:String) {
//        println!("Playing movie {}",name);
//     }
//  }
//  use movies::play;
//  fn main(){
//     play("Herold and Kumar ".to_string());
//  }

pub mod movies {
    pub mod english {
       pub mod comedy {
          pub fn play(name:String) {
             println!("Playing comedy movie {}",name);
          }
       }
    }
 }
 use movies::english::comedy::play; 
 // importing a public module
 
 fn main() {
    // short path syntax
    play("Herold and Kumar".to_string());
    play("The Hangover".to_string());
 
    //full path syntax
    movies::english::comedy::play("Airplane!".to_string());
 }