use rocket_okapi::openapi;
use rocket::serde::{json::Json};

use fbsim_core::league::League;

/// Add a team to a league
///
/// Given a league spec, add a new team to the league
#[openapi(tag = "LeagueTeams")]
#[post("/v1/stateless/leagues/teams/new", format="application/json", data="<league>")]
pub fn add_team(league: Json<League>) -> Json<League> {
    // Get the league from the request body
    let mut league: League = league.into_inner();

    // Add a team to the league
    league.add_team();

    // Return the league as JSON
    Json(league)
}
