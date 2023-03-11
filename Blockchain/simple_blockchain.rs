
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
// i define a struct named 'Block', which encapsulates the properties of a block (index, timestamp, data, previous_hash, hash).
