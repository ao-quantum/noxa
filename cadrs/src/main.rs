#[macro_use] extern crate rocket;

mod fsrs;
mod db;
mod endpoints;

use db::{fetch_user_by_sessionid, User};
use endpoints::{
    create_card::create_card, delete_card::delete_card, get_cards::get_cards, next_card::next_card, review::review_card, stats::stats
};

use rocket::{http::{Method, Status}, request::{FromRequest, Outcome}, serde::json::Json, Request};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPoolOptions, MySqlPool};

// Shared state for Rocket
// Holds just the database connection pool
struct Shared {
    db: MySqlPool,
}

// Standard response structure for the API
#[derive(Serialize, Deserialize, Debug)]
struct CADRSResponse<D> {
    success: bool,
    message: Option<String>,
    data: Option<D>,
}

// Custom request guard to extract the user from the request
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, Self::Error> {
        let sessionid = request.headers().get_one("Authorization"); // get Authorization header
        if sessionid.is_none() { // If none, then reject request
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let sessionid = sessionid.unwrap(); // unwrap sessionid

        let user = match fetch_user_by_sessionid(&request.rocket().state::<Shared>().unwrap().db, sessionid).await { // get user from the sessionid
            Ok(user) => user, // return user if found
            Err(_) => return Outcome::Error((Status::InternalServerError, ())), // reject if no user found for the sesssionid (either invalid sessionid or it expired, probably)
        };

        if user.is_some() { // If user found, return it
            return Outcome::Success(user.unwrap());
        } else { // If no user, reject the request
            return Outcome::Error((Status::Unauthorized, ()));
        }
    }
}

// Error handler for all HTTP errors
#[catch(default)]
fn default_catch(status: Status, _req: &Request) -> Json<CADRSResponse<()>> {
    Json(CADRSResponse { // Return error in standard JSON response format
        message: Some(format!("{}", status)),
        success: false,
        data: None,
    })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok(); // Load environment variables from .env file

    // Create the connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5) // Set max connections to 5
        .connect(std::env::var("DATABASE_URL").unwrap().as_str()) // Get the db URL from env vars
        .await
        .expect("Failed to connect to database"); // Panic if connection fails

    // CORS configuration
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all()) // Allow all origins
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(), // Allow specific HTTP methods
        )
        .allow_credentials(true); // Allow credentials to be sent with requests

    // Start the rocket server
    let _rocket = rocket::build()
        .mount("/cards", routes![ // Mount endpoints
            get_cards,
            create_card,
            delete_card,
            next_card,
            review_card,
            stats,
        ])
        .register("/", catchers![default_catch]) // Mount error handlers
        .manage(Shared { // Mount shared state
            db: pool,
        })
        .attach(cors.to_cors().unwrap()) // Attach CORS.
        .launch()
        .await?;

    Ok(())
}
