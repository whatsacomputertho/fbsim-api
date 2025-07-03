use rocket_okapi::openapi;
use rocket::serde::{json::Json};

use fbsim_core::boxscore::BoxScore;
use fbsim_core::matchup::FootballMatchup;
use fbsim_core::sim::BoxScoreSimulator;

/// Simulate a football game
///
/// Given a spec defining the matchup between the home and away team, this
/// endpoint simulates a game between the two teams.
#[openapi(tag = "Games")]
#[post("/game/sim", format="application/json", data="<matchup>")]
pub fn game_sim(matchup: Json<FootballMatchup>) -> Json<BoxScore> {
    // Get the matchup from the request body
    let matchup: FootballMatchup = matchup.into_inner();

    // Simulate the game
    let box_score_sim = BoxScoreSimulator::new();
    let mut rng = rand::thread_rng();
    let box_score = box_score_sim.sim(
        matchup.home_team(),
        matchup.away_team(),
        &mut rng
    ).unwrap();

    // Return the box score as a JSON string
    Json(box_score)
}
