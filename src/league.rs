pub mod team;

use rocket_okapi::openapi;
use rocket::serde::{json::Json};

use fbsim_core::league::League;

/// Create a new league
///
/// Create a new default league containing no teams or seasons
#[openapi(tag = "Leagues")]
#[get("/v1/stateless/leagues/new", format="application/json")]
pub fn get_league() -> Json<League> {
    // Create a new league and return as JSON
    let league = League::new();
    Json(league)
}
