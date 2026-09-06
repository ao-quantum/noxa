use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};

use crate::fsrs::{self, Grade, D, R, S};

/**
 * Guides used 
 * https://medium.com/@mikecode/rust-sqlx-mysql-c6f0f28fc4e5
 * https://gist.github.com/jeremychone/34d1e3daffc38eb602b1a9ab21298d10
 */

// Add an alias for the integer id type in the database, so that it can be easily changed later if needed
pub type IntIdType = i32;

// Struct for `cards` table`
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: IntIdType,
    pub userid: IntIdType,
    pub front: String,
    pub back: String,
    pub s: Option<f64>,
    pub d: Option<f64>,
    pub r: Option<f64>,
    pub last_review: Option<chrono::NaiveDateTime>,
    pub createdat: chrono::NaiveDateTime,
    pub updatedat: chrono::NaiveDateTime,
}

// Struct for `cardrecalls` table
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CardRecall {
    pub id: IntIdType,
    pub cardid: IntIdType,
    pub grade: i32,
    pub createdat: chrono::NaiveDateTime,
}

// Struct for `users` table
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User { 
    pub id: IntIdType,
    pub email: String,
    pub username: String,
    pub password: String,
    pub createdat: chrono::NaiveDateTime,
    pub updatedat: chrono::NaiveDateTime,
}

// Struct for `sessions` table
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub userid: IntIdType,
    pub ip: String,
    pub expire: chrono::NaiveDateTime,
    pub createdat: chrono::NaiveDateTime,
}

/**
 * Fetch all the cards for a user
 */
pub async fn fetch_cards(db: &MySqlPool, user_id: i32) // db to connect, user_id to fetch cards for
-> Result<Vec<Card>, sqlx::Error> { // Result, Vec<Card> if Ok, sqlx::Error if Err occured
    Ok(sqlx::query_as("SELECT * FROM cards WHERE userid = ? LIMIT 10000;") // Limit the query so that memory isn't overloaded
        .bind(user_id)
        .fetch_all(db)
        .await?)
}

/**
 * Fetch the cards, and update all the cards' r values to the current retrievability
 */
pub async fn fetch_cards_current_r(db: &MySqlPool, user_id: IntIdType) // db to connect, user_id to fetch cards for
-> Vec<Card> { // Result, Vec<Card> if Ok, sqlx::Error if Err occured
    let cards = fetch_cards(db, user_id).await.unwrap();
    return cards
        .iter()
        .cloned()
        .map(|mut c| {
            if c.last_review.is_none() {
                return c;
            }

            let last_review = c.last_review.unwrap().and_utc().timestamp();
            let time = (chrono::Utc::now().timestamp() - last_review) as f64 / 86400.0;
            let stability = c.s.unwrap();
            c.r = Some(fsrs::retrievability(time, stability));
            c
        })
        .collect();
}

/**
 * Fetch card by a given id
 */
pub async fn fetch_card_by_id(db: &MySqlPool, card_id: i32) // db to connect, and card_id to fetch
-> Result<Option<Card>, sqlx::Error> { // Result, Optional Card if Ok, sqlx::Error if Err occured
    Ok(sqlx::query_as("SELECT * FROM cards WHERE id = ? LIMIT 1;")
        .bind(card_id)
        .fetch_optional(db)
        .await?)
}

/**
 * Update a card's d, s, r, and last_review values
 * Prepares a statement, and binds each argument to the statement, and then executes it.
 */
pub async fn update_card_dsrl(db: &MySqlPool, card_id: i32, d: D, s: S, r: R, last_review: chrono::NaiveDateTime) // db to connect, card_id to update accordingly, d, s, r, and last_review values to update 
-> Result<(), sqlx::Error> { // Result, () if Ok, sqlx::Error if Err occured
    sqlx::query("UPDATE cards SET d = ?, s = ?, r = ?, last_review = ? WHERE id = ?")
        .bind(d)
        .bind(s)
        .bind(r)
        .bind(last_review)
        .bind(card_id)
        .execute(db) // run the query not expecting any returned rows
        .await?;

    Ok(())
}

/**
 * Record an instance of a card being recalled (creates a cardrecall row)
 */
pub async fn create_card_recall(db: &MySqlPool, card_id: i32, grade: Grade) // db to connect, card_id to recall, and grade of the recall
-> Result<(), sqlx::Error> { // Result, () if Ok, sqlx::Error if Err occured
    sqlx::query("INSERT INTO cardrecalls (cardid, grade) VALUES (?, ?)")
        .bind(card_id)
        .bind(f64::from(grade) as i32)
        .execute(db)
        .await?;

    Ok(())
}

/**
 * Create a card in the database
 */
pub async fn create_card(db: &MySqlPool, user_id: IntIdType, front: String, back: String) // db to connect, user_id to create the card for, front and back of the card
-> Result<(), sqlx::Error> { // Result, () if Ok, sqlx::Error if Err occured
    sqlx::query("INSERT INTO cards (userid, front, back) VALUES (?, ?, ?);")
        .bind(user_id)
        .bind(front)
        .bind(back)
        .execute(db)
        .await?;

    Ok(())
}

/**
 * Deletes a card from the database.
 * Note that if the card didn't exist to begin with, there is no error thrown.
 */
pub async fn delete_card(db: &MySqlPool, card_id: IntIdType) // db to connect, and card_id to delete
-> Result<(), sqlx::Error> { // Result, () if Ok, sqlx::Error if Err occured
    sqlx::query("DELETE FROM cards WHERE id = ?")
        .bind(card_id)
        .execute(db)
        .await?;

    Ok(())
}

/**
 * Fetch a user with a given session id
 * The user bound to the session with the given session_id is returned
 */
pub async fn fetch_user_by_sessionid(db: &MySqlPool, session_id: &str) // db to connect, and session_id to fetch user for
-> Result<Option<User>, sqlx::Error> { // Result, Optional User if Ok, sqlx::Error if Err occured
    Ok(sqlx::query_as("SELECT users.* FROM sessions JOIN users ON users.id = sessions.userid WHERE sessions.id = ?")
        .bind(session_id)
        .fetch_optional(db)
        .await?)
}
