
use std::fmt::{Display, Formatter, Result};
// i am specifying that i'll be using the 'fmt' module from Rust's standard library.
// This module contains useful functions, particularly for printing to the console.

struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}
// I define a struct named 'Block', which encapsulates the properties of a block (index, timestamp, data, previous_hash, hash).

impl Block {
    fn new(index: u32, timestamp: u128, data: String, previous_hash: String, hash: String) -> Block {
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}
// I define a constructor function for the 'Block' struct, which makes it easier to create new blocks.


impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Block {}: {} (Timestamp: {}, Hash: {}, Previous Hash: {})",
            self.index, self.data, self.timestamp, self.hash, self.previous_hash
        )
    }
}
// By implementing the 'Display' trait, I specify how the 'Block' struct should be displayed when printed to the console.
