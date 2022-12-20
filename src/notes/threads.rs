use std::sync::{Arc, Mutex};
use std::thread;

use crate::notes::node_struct::DASNode;

/*
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. 
You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context. 
Unlike functions, closures can capture values from the scope in which they’re defined.

Forcing a closure to take ownership of the values it uses, via "move", is useful when passing a closure to a new 
thread to move the data so that it's owned by said thread.
*/

fn threads() {
    let my_node = DASNode::new();
    let mut my_node = Arc::new(Mutex::new(my_node));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let thread_node = Arc::clone(&my_node); 
        let handle = thread::spawn(move || {
            let mut thread_node = thread_node.lock().unwrap(); 
            println!("Thread node: {:?}", thread_node);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();   
    }
    println!("My node: {:?}", my_node);

    /*
    Rc<T>'s, aka Reference Counted types, are smart pointers that allow for a single
    value to have multiple owners.

    Arc<T>'s, aka Atomic Reference Counted types, are smart pointers that allow for 
    a single value to have multiple owners.  ***These multiple owners can be shared across multiple threads***
    
    */
}

pub fn run_notes_threads() {
    let answer = crate::notes::threads::threads();
    answer
}