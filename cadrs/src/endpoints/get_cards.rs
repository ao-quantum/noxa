use rocket::{serde::json::Json, State};

use crate::{db::{self, fetch_cards}, CADRSResponse, Shared};

#[get("/")]
pub async fn get_cards(shared: &State<Shared>, user: db::User) // Accept shared state and user data
-> Json<CADRSResponse<Vec<db::Card>>> { // Return JSON object containing array of cards
    let cards = fetch_cards(&shared.db, user.id).await.unwrap(); // get all cards and unwrap the result

    Json(CADRSResponse { // return JSON response
        success: true,
        message: None,
        data: Some(cards), // return cards in the response as an array
    })
}
