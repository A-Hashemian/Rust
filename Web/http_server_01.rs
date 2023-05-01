
use reqwest::Client;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
/*
This line is a Rust macro that sets up a tokio runtime, which allows us to run asynchronous code
The #[tokio::main] attribute on the main function tells Rust to use tokio as the runtime for this program

The async keyword indicates that the function is an asynchronous function that returns a future.
The Result<() Box<dyn std::error::Error>> return type specifies that the function returns a Result with an empty success value (()) and a Box containing any type that implements the std::error::Error trait as an error value
*/
