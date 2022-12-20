use discv5_overlay::portalnet::discovery::Discovery;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DASNode {
    discovery: Arc<Discovery>,
    libp2p: String,
    samples: [u8; 8],
    handled_ids: String,
    overlay: String,
}
/*
    Begin to import fields from other crates!

    First 
*/

impl DASNode {
    // pub fn new(discovery: Arc<Discovery>) -> Self {
    pub fn new() -> Self {
        Self {
            discovery,
            libp2p: String::from("None"),
            samples: [0; 8],       // Correct number of samples???
            handled_ids: String::from("None"),
            overlay: String::from("None"),
        }
    }
}