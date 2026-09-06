// Guides used:
// https://expertium.github.io/Algorithm.html
// https://borretti.me/article/implementing-fsrs-in-100-lines

use serde::{Deserialize, Serialize};

// Stability of a card. Time taken for R to go from 1 to 0.9. From 0 to infinity.
pub type S = f64;

// Retrievability of a card. Probability of recalling the memory.
pub type R = f64;

// Difficulty, from 1 to 10, inclusive
pub type D = f64;

// Various weights for the algorithm. All these values have been fetched from the FSRS github.
// The numbers below are optimal values. Found on https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Algorithm
const W: [f64; 19] = [0.40255, 1.18385, 3.173, 15.69105, 7.1949, 0.5345, 1.4604, 0.0046, 1.54575, 0.1192, 1.01925, 1.9395, 0.11, 0.29605, 2.2698, 0.2315, 2.9898, 0.51655, 0.6621];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Grade {
    Forgot, // Grade for forgetting
    Hard,   // Grade for hard recall
    Good,   // Grade for good recall
    Easy    // Grade for easy recall
}

/**
 * Allow parsing from f64 to grade
 */
impl From<Grade> for f64 {
    fn from(value: Grade) -> f64 {
        match value {
            Grade::Forgot => 1.0, // Map Forgot to 1.0
            Grade::Hard => 2.0,   // Map Hard to 2.0
            Grade::Good => 3.0,   // Map Good to 3.0
            Grade::Easy => 4.0    // Map Easy to 4.0
        }
    }
}

/**
 * Allow parsing from grade to f64
 */
impl From<f64> for Grade {
    fn from(value: f64) -> Grade {
        match value {
            1.0 => Grade::Forgot, // Map 1.0 to Forgot
            2.0 => Grade::Hard,   // Map 2.0 to Hard
            3.0 => Grade::Good,   // Map 3.0 to Good
            4.0 => Grade::Easy,   // Map 4.0 to Easy
            _ => panic!("Invalid grade value") // Panic for invalid values
        }
    }
}

const F: f64 = 19.0 / 81.0; // Constant F for retrievability calculation
const C: f64 = -0.5;        // Constant C for retrievability calculation

pub const TARGET_R: f64 = 0.9; // Target retrievability value

/**
 * Function for the retrievability of a card.
 */
pub fn retrievability(time: f64, stability: S) -> f64 {
    (1.0 + F * time / stability).powf(C) // Calculate retrievability
}

/**
 * Function for the initial stability of a card based on the first rating/grade.
 */
pub fn s_initial(g: Grade) -> f64 {
    let d = f64::from(g) - 1.0; // Convert grade to index
    return W[d as usize];       // Return corresponding weight
}

/**
 * Next stability value of a card
 * d - old difficulty value before the review
 * s - old stability value
 * r - retrievability at review
 * g - grade achieved
 */
pub fn s_next(d: D, s: S, r: R, g: Grade) -> f64 {
    if g == Grade::Forgot {
        return s_next_forgot(d, s, r); // Use forgot logic if grade is Forgot
    } else {
        return s_next_ok(d, s, r, g); // Use ok logic otherwise
    }
}

/**
 * Next stability value of a card, given that it was not forgotten.
 * Do not use this on its own unless necessary. Use s_next instead.
 */
fn s_next_ok(d: D, s: S, r: R, g: Grade) -> f64 {
    let fd = 11.0 - d; // difficulty factor
    let fs = s.powf(-1.0 * W[9]); // stability factor
    let fr = (W[10] * (1.0 - r)).exp() - 1.0; // retrievability factor
    let w15 = match g { // weight for Hard grade
        Grade::Hard => W[15],
        _ => 1.0,
    };
    let w16 = match g { // weight for Easy grade
        Grade::Easy => W[16],
        _ => 1.0,
    };

    let s_inc = 1.0 + w15 * w16 * W[8].exp() * fd * fs * fr; // stability increment

    return s * s_inc; // Return new stability
}

/**
 * Next stability value of a card if it is forgotten
 * The r to be used is the old r, not the new one.
 * Do not use this on its own unless necessary. Use s_next instead.
 */
fn s_next_forgot(d: D, s: S, r: R) -> f64 {
    // Ensure stability does not increase
    return f64::min( // choose lowest value out of
        W[11] * d.powf(-1.0 * W[12]) * ((s + 1.0).powf(W[13]) - 1.0) * (W[14] * (1.0 - r)).exp(), // next stability for forgot case
        s // Ensure stability does not increase
    );
}

/**
 * Function to restrict difficulty to its valid range
 */
fn d_restrict(d: D) -> f64 {
    return d.clamp(1.0, 10.0); // Clamp difficulty between 1 and 10
}

/**
 * Function for the initial/first time difficulty of a card.
 */
pub fn d_initial(g: Grade) -> f64 {
    let g = f64::from(g); // Convert grade to f64
    return d_restrict(W[4] - (W[5] * (g - 1.0)).exp() + 1.0); // Calculate initial difficulty
}

/**
 * Next difficulty value of a card
 * d is the current/old difficulty
 */
pub fn d_next(d: D, g: Grade) -> f64 {
    let d = f64::from(d); // convert difficulty to f64
    let delta_d = -1.0 * W[6] * (f64::from(g) - 3.0); // calculate delta d
    
    return d_restrict( // restrict difficulty to valid range
        W[7] * d_initial(Grade::Easy) + (1.0 - W[7]) * (d + delta_d * ((10.0 - d)/9.0)) // next difficulty
    );
}
