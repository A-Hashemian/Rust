
use reqwest::Client;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
/*
This line is a Rust macro that sets up a tokio runtime, which allows us to run asynchronous code
The #[tokio::main] attribute on the main function tells Rust to use tokio as the runtime for this program

*/
