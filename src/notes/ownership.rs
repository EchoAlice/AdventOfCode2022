
/*
When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) 
and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

*The main purpose of ownership is to manage heap data*


Ownership Rules:
    1. Each value in Rust has an owner
    2. There can only be one owner at a time
    3. When the owner goes out of scope, the value will be dropped
*/

fn ownership() {
    // Binds the value 5 to x, then makes a copy of the value in x and binds it to y 
    let x = 5;
    let y = x;
    // println!("x: {}", x); 
    // println!("y: {}", y); 
    
    /*
        S2 recieves the smart pointer data that IS s1;  S1 then drops the smart pointer 
        to protect against double freeing memory.  This is known as a "move"
    */
    let s1 = String::from("Echo");
    let s2 = s1;
    // println!("S2: {}", s2);
}

pub fn run_notes_ownership() {
    let answer = crate::notes::ownership::ownership();
    answer
}