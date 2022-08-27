### Learning Rust with Solana

Installation
-------------
- Install Rust
- Install Crates Extenstions in VS Code
- Install Rust-Analyzer Extenstions in VS Code

###### 1 First Rust Program
-----------------------------
Goto the Terminal use command
```
cargo new hello-world --bin
```

- Add gitignore
- Understand Cargo.toml
- Understand main.rs file is the entry point of your pogram

Run your Pogram
```
cargo run
```

Find the Binanry and run your binary file.

###### 2 Variable & Mutability 
-------------------------------
let x = "Hello!";
- let variable declation
- x variable nam
- "Hello!" initial value

let mut x = 8;
x = 2;
- can't change value of variable by default you need to use 'mut'

###### 3 Types 
---------------

let x: i32 = 8;

x.print_type(); // print_type to know the variable type

###### 4 Tuples 
----------------
'''
let a =(1, "hello", false);
println!("The first value : {}", a.0);
'''

Array
-----
'''
let a = [1,2,3];
let a = [0; 10];
let a: [i32; 10] = [0; 10];

let mut a = [1,2,3];
'''

###### 5 Control Flow 
----------------------

'''
if num == 1 {
    //true
}
'''

'''
if num == 1 {
    // true
}
else{
    // false
}
'''

'''
if num == 1{
    // true
} else if num == 2{
    // true
}else{
    // false
}
'''

###### 5 Loops
---------------

'''
loo{
    condition match add break;
}
'''

'''
while x<500 {

}
'''

'''
for x in 0..10{
   // print here 0 to 9
}
'''

'''
for y in 0..=9{
    //print 0 to 9
}
'''