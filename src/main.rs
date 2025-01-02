mod matchup;

use crate::matchup::FootballMatchup;

use fbsim_core::boxscore::BoxScore;
use fbsim_core::sim::BoxScoreSimulator;
use rand;
use rocket::serde::{json::Json};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

#[macro_use] extern crate rocket;

/// Simulate a football game
///
/// Given a spec defining the matchup between the home and away team, this
/// endpoint simulates a game between the two teams.
#[openapi(tag = "Games")]
#[post("/game/sim", format="application/json", data="<matchup>")]
fn game_sim(matchup: Json<FootballMatchup>) -> Json<BoxScore> {
    // Get the matchup from the request body
    let matchup: FootballMatchup = matchup.into_inner();

    // Simulate the game
    let box_score_sim = BoxScoreSimulator::new();
    let mut rng = rand::thread_rng();
    let box_score = box_score_sim.sim(
        matchup.home_team(),
        matchup.away_team(),
        &mut rng
    );

    // Return the box score as a JSON string
    Json(box_score)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![game_sim])
        .mount("/openapi", openapi_get_routes![game_sim])
        .mount("/openapi", make_swagger_ui(&SwaggerUIConfig {
            url: "openapi.json".to_owned(),
            ..Default::default()
        }))
}
