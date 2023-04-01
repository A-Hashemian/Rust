
#![feature(proc_macro_hygiene, decl_macro)]


// tells Rust to import the rocket crate and allow the use of macros defined in the crate
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// creates a new instance of the Rocket application, mounts the index route at the root ("/"), and launches the server
fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}


