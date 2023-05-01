
use reqwest::Client;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
/*
This line is a Rust macro that sets up a tokio runtime, which allows us to run asynchronous code
The #[tokio::main] attribute on the main function tells Rust to use tokio as the runtime for this program

The async keyword indicates that the function is an asynchronous function that returns a future.
The Result<() Box<dyn std::error::Error>> return type specifies that the function returns a Result with an empty success value (()) and a Box containing any type that implements the std::error::Error trait as an error value
*/


let urls = vec![
    "https://www.google.com",
    "https://www.github.com",
    "https://www.rust-lang.org",
];

// This line creates a vector of URLs that we want to send GET requests to



// This line creates a new Client instance from the reqwest crate that we'll use to send the HTTP requests
let client = Client::new();



let responses = futures::future::join_all(
    urls.iter().map(|url| async move {
        let response = client.get(url).send().await?;
        response.text().await.map_err(Into::into)
    })
).await;

/*
this section of code uses the futures::future::join_all function to create a future that sends all the requests asynchronously
the join_all function takes an iterator of futures and returns a future that resolves to a vector of the results of each future
each future in the iterator sends a GET request to a single URL and returns the response body as a string

*/




for response in responses {
    match response {
        Ok(body) => println!("Response body: {}", body),
        Err(e) => eprintln!("Error: {}", e),
    }
}

