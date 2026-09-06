use rocket::{serde::json::Json, State};
use serde::Serialize;

use crate::{db::{self, fetch_cards_current_r}, fsrs, CADRSResponse, Shared};

// Define the structure of the response data
#[derive(Serialize)]
pub struct Stats {
    total: u64,
    due: u64,
    not_reviewed_yet: u64,
    stability_mean: Option<f64>,
}

#[get("/stats")] // Set route to GET /stats
pub async fn stats(shared: &State<Shared>, user: db::User) // Take shared state and user data from request guard
-> Json<CADRSResponse<Stats>> { // Return JSON response with stats data
    let cards = fetch_cards_current_r(&shared.db, user.id).await; // fetch all cards with live r values

    let total = cards.len() as u64; // total number of cards

    // A card is due if it's retrieval probability is less than TARGET_R
    // filter all the cards with r < TARGET_R for the due count
    let due = cards.iter().filter(|card| {
        if card.last_review.is_none() {
            return true; // if not reviewed, then it is due
        }
        
        if card.r.unwrap() < fsrs::TARGET_R {
            return true; // if r is less than TARGET_R, then it is due
        }

        return false; // else for all cases, not due
    }).count() as u64; // cast to u64

    let not_reviewed_yet = cards.iter().filter(|card| card.last_review.is_none()).count() as u64; // count up the cards which have no last_review
    let stability_vec = cards // array of the stability values
        .iter() // iterate over the cards
        .filter(|c| c.s.is_some()) // filter out the cards which have no stability
        .map(|c| c.s.unwrap().clone()) // map the cards to their stability values
        .collect::<Vec<f64>>(); // collect the values into a f64 vec

    let stability_mean = stability_vec
        .iter()
        .sum::<f64>() / stability_vec.len() as f64;

    let stability_mean_rounded = (stability_mean * 1000.0).round() / 1000.0; // round to 3 decimal places

    // Return the JSON response with the stats data
    Json(CADRSResponse {
        success: true,
        message: None,
        data: Some(Stats {
            total,
            due,
            not_reviewed_yet,
            stability_mean: Some(stability_mean_rounded),
        }),
    })
}
