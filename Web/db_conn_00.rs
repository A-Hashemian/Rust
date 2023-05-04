
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // Open a database connection
    let conn = Connection::open("example.db")?;
  
      // Create a table called "person"
    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER NOT NULL
                  )",
        [],
    )?;
  
       // Insert a record into the "person" table
    conn.execute(
        "INSERT INTO person (name, age)
                  VALUES (?1, ?2)",
        ["Alice", 42],
    )?;
    
    
   // Close the database connection
    Ok(()) 
    
  }
