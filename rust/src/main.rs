#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rusqlite::{Connection, Result};

// Define a structure to represent the data sent by the Flutter app
#[derive(Debug, serde::Deserialize)]
struct Data {
    data: String,
}

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;


//TODO: explanation: https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


// Define a Rocket route to receive data from the Flutter app
#[post("/endpoint", data = "<data>")]
fn receive_data(data: Json<Data>) -> &'static str {
    // Access the data received by <the endpoint
    let received_data = &data.data;
    
    // Print the received data
    println!("Data received: {}", received_data);
    
    match store_data(&data.data) {
        Ok(_) => "Data stored successfully!",
        Err(_) => "Failed to store data.",
    }
}
#[options("/endpoint")]
fn options_endpoint() -> &'static str {
    // Without this endpoint rocket would get OPTIONS request to the /endpoint route, 
    // but there is no matching route defined to handle OPTIONS requests.

    // The OPTIONS request is part of the CORS (Cross-Origin Resource Sharing) mechanism and 
    // is typically sent by the browser as a preflight request to check if the actual request 
    // (e.g., POST, GET) is safe to send to the server.
    
    "" 
}

// Function to store data in SQLite database
fn store_data(data: &str) -> Result<()> {
    let conn = Connection::open("example.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (id INTEGER PRIMARY KEY, value TEXT)",
        [],
    )?;
    conn.execute(
        "INSERT INTO data (value) VALUES (?)",
        &[&data],
    )?;
    Ok(())
}

// Rocket fairing to catch errors and display them as HTTP responses
#[catch(500)]
fn internal_error() -> &'static str {
    "Internal server error"
}


// Main function to launch the Rocket application
#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![receive_data, options_endpoint])
}