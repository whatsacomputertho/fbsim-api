use fbsim_core::boxscore::BoxScore;
use fbsim_core::matchup::FootballMatchup;
use fbsim_core::sim::BoxScoreSimulator;
use rand;
use rocket::http::Method;
use rocket::serde::{json::Json};
use rocket_cors::{catch_all_options_routes, AllowedOrigins, CorsOptions};
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
    ).unwrap();

    // Return the box score as a JSON string
    Json(box_score)
}

// Set CORS headers on responses
fn get_cors() -> CorsOptions {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
}

// Launch the rocket API server
#[launch]
fn rocket() -> _ {
    let cors = get_cors().to_cors().unwrap();
    rocket::build()
        .mount("/", routes![game_sim])
        .mount("/", catch_all_options_routes())
        .mount("/openapi", openapi_get_routes![game_sim])
        .mount("/openapi", make_swagger_ui(&SwaggerUIConfig {
            url: "openapi.json".to_owned(),
            ..Default::default()
        }))
        .attach(cors.clone())
        .manage(cors)
}
