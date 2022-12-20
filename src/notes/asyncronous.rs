use crate::notes::node_struct::DASNode;

use discv5::Discv5;
use discv5_overlay::portalnet::discovery::Discovery;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;


/*
    Big Questions:
        What should I try to model with the overlay?
*/

/* 
This function will be executed by the Tokio runtime.  Runtimes are responsible for polling our top level futures 
and running them until completion.  Runtimes are also responsible for running multiple futures in parallel.

Spin up 10 DASNodes that have their own Discovery Protocol.  
Try adding information to their routing tables!  Check out APIs for the discv5 crate





let discovery = Arc::new(Discovery::new_raw(discv5, Default::default()));      Line 219 main.rs
    What is portal_config?

*/
#[tokio::main]
async fn create_nodes () {
    let discovery = "We need to instantiate the server/service"; 
    
    let my_node = DASNode::new();
    let mut my_node = Arc::new(Mutex::new(my_node));
    
    // Spin up new tasks.  
    let mut handles = Vec::new();
    for i in 0..10 {
        let task_node = Arc::clone(&my_node); 
        let handle = tokio::spawn(async move {
            let mut task_node = task_node.lock().unwrap();
            println!("Peer number: {i}");
            
            // Create 10 DASNodes 
            // node.new(); 
        });
        handles.push(handle);
    } 

    for handle in handles {
        handle.await.unwrap();
    }
   
    println!("Servers spun up:");
}



async fn ping_node() {
    println!("Ping Node")
}



pub fn run_notes_async() {
    let answer = crate::notes::asyncronous::create_nodes();
    answer
}