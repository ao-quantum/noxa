use rocket::{http::Status, serde::json::Json, State};

use crate::{db::{self, IntIdType}, CADRSResponse, Shared};

#[delete("/<id>")]
pub async fn delete_card(shared: &State<Shared>, id: IntIdType) // Take shared state and id from URL parameter
-> Result<Json<CADRSResponse<()>>, Status> { // Return JSON response with no data
    match db::delete_card(&shared.db, id).await { // Call delete card function and match Result
        Ok(_) => return Ok(Json(CADRSResponse { // Return JSON response with empty body on Ok outcome
            success: true,
            message: None,
            data: None,
        })),
        Err(err) => { // On error result
            eprintln!("{:?}", err); // print error to stderr
            return Err(Status::InternalServerError); // return error 500
        },
    };
}
