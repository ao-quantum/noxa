use rocket::{serde::json::Json, State};
use serde::Deserialize;

use crate::{db::{self, create_card_recall, fetch_card_by_id, IntIdType}, fsrs::{d_initial, d_next, retrievability, s_initial, s_next, Grade, TARGET_R}, CADRSResponse, Shared};

// Card review options which the server accepts
#[derive(Deserialize, Debug)]
pub struct ReviewCardOptions {
    card_id: IntIdType,
    grade: u8, // Use u8, rather than f64, to represent the rating to save memory and ensure it is a whole number
}

#[patch("/review", data = "<opts>")] // Route to PATCH /review, and assign request body to opts param
pub async fn review_card(shared: &State<Shared>, opts: Json<ReviewCardOptions>) // Take shared state and JSON object from request body following ReviewCardOptions
-> Json<CADRSResponse<()>> { // Return JSON response with no data
    let card = match fetch_card_by_id(&shared.db, opts.card_id).await.unwrap() { // get the card by id and unwrap the Option
        Some(card) => card, // If card is found, then return it to assign to `card`
        None => {
            return Json(CADRSResponse { // If no card found return error response
                success: false,
                message: Some("Card not found".to_string()),
                data: None,
            });
        },
    };

    let grade = Grade::from(f64::from(opts.grade));

    if card.r.is_none()
    || card.s.is_none()
    || card.d.is_none()
    || card.last_review.is_none(){ // if any of d, s, r, or last_review is None
        // As a reminder:
        // - s is stability is the number of days it takes for r to go from 1 to 0.9
        // - d is difficulty of the card
        // - r is retrievability, probability of recalling the card at a given point in time since last review

        let new_s = s_initial(grade); // calculate new s
        let new_d = d_initial(grade); // calculate new d
        let new_r = TARGET_R; // set r to the target retrievability
        let now = chrono::Utc::now(); // get current time
        
        // Last recall time (which is the first recall)
        let last_review = now.naive_utc(); // set last review to now

        db::update_card_dsrl( // update the card with the new values
            &shared.db,
            card.id,
            new_d,
            new_s,
            new_r,
            last_review,
        ).await.unwrap();
    } else { // else if all values are present
        // Calculate time since last recall
        let now = chrono::Utc::now();
        
        let d = card.d.unwrap(); // unwrap d
        let s = card.s.unwrap(); // unwrap s

        // Calculate days difference between now and last review
        // First calculate seconds difference
        // We cast to f64 now so we can use it later in float division
        let secs_since_last_review = (now.timestamp() - card.last_review.unwrap().and_utc().timestamp()) as f64;
        let days_since_last_review = secs_since_last_review / 86400.0; // Divide by 86400 to get days, since 86400 seconds in a day

        let new_r = retrievability(days_since_last_review, s); // calculate new r, ie the current retrievability
        let new_s = s_next(d, s, new_r, grade); // calculate new s
        let new_d = d_next(d, grade); // calculate new d

        db::update_card_dsrl( // update the card with the new values
            &shared.db,
            card.id,
            new_d,
            new_s,
            new_r,
            now.naive_utc(), // last_review
        ).await.unwrap();
    }

    // Create cardrecall record for this review
    create_card_recall(&shared.db, card.id, grade).await.unwrap();

    // Return success response
    Json(CADRSResponse {
        success: true,
        message: None,
        data: None,
    })
}
