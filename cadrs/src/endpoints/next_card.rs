use rocket::{serde::json::Json, State};

use crate::{db::{self, fetch_cards_current_r}, CADRSResponse, Shared};

#[get("/next")] // Define the route to GET /next
pub async fn next_card(shared: &State<Shared>, user: db::User) // Take shared state and user data from request guard
-> Json<CADRSResponse<db::Card>> { // Return JSON response with card data
    // Priority of which card to recommend next in fsrs:
    // - Cards that have not been reviewed yet
    // - Cards that have been reviewed, in order of recent to later review

    // The cards will be sorted in null, then asc order of i
    let mut cards = fetch_cards_current_r(&shared.db, user.id).await; // get all cards and unwrap the result

    // sort the cards next
    // order by null last_review, then asc r
    cards.sort_by(|a, b| {
        // note that we compare r instead of last_review
        // if there is no last_review, it cannot have an r
        // so a presence of r is linked to the presence of last_review
        // we check r though because we are unwrapping it later, so we check for last_review, but also that r exists at the same time (in case there was some catastrophic data nonsense)

        if a.r.is_none() && b.r.is_none() {
            return std::cmp::Ordering::Equal; // If both cards have no r, they are equal
        } else if a.r.is_none() {
            return std::cmp::Ordering::Less; // if a has no r, a comes before b
        } else if b.r.is_none() {
            return std::cmp::Ordering::Greater; // if b has no r, b comes before a
        }

        // unwrap the r values (Option)
        let a_r = a.r.unwrap();
        let b_r = b.r.unwrap();

        if a_r < b_r {
            return std::cmp::Ordering::Less; // If a's r is less than b's r, a comes before b
        } else if a_r > b_r {
            return std::cmp::Ordering::Greater; // If a's r is greater than b's r, b comes before a
        } else {
            return std::cmp::Ordering::Equal; // If a's r is equal to b's r, they are equal
        }
    });

    if cards.len() == 0 { // if there are no cards, return response with empty data
        Json(CADRSResponse {
            success: true,
            message: None,
            data: None,
        })
    } else {
        let card = cards.first().unwrap(); // get the first card in the sorted list
        Json(CADRSResponse { // return JSON response
            success: true,
            message: None,
            data: Some(card.clone()), // clone the `card` because `card` is a reference, and to return it we need to own it
        })
    }

}
