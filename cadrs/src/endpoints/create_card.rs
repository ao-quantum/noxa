use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;

use crate::{db, CADRSResponse, Shared};

#[derive(Deserialize)]
pub struct CardCreateOptions {
    front: String,
    back: String,
}

/**
 * Expects a JSON object in the request body with the following fields:
 * - front
 * - back
 */
#[post("/", data = "<opts>")] // Set the route to POST / and set request body to come into the opts parameter
pub async fn create_card(shared: &State<Shared>, user: db::User, opts: Json<CardCreateOptions>) // Take shared state, user data from request guard, and JSON object from request body
-> Result<Json<CADRSResponse<()>>, Status> { // Return result of JSON res with no data or custom status
    let card = db::create_card(&shared.db, user.id, opts.front.clone(), opts.back.clone()).await; // get the cards

    match card { // match result of create card function
        Ok(_) => return Ok(Json(CADRSResponse { // return Ok with Ok status and JSON response
            data: None,
            success: true,
            message: None,
        })),
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(Status::InternalServerError) // return error 500 if creation failed
            // this error will pass through the default_catch function so no need to handle proper json error
        },
    }
}
