mod game;
mod league;

use rocket::http::Method;
use rocket_cors::{catch_all_options_routes, AllowedOrigins, CorsOptions};
use rocket_okapi::{openapi_get_routes, swagger_ui::*};

use crate::game::{game_sim, okapi_add_operation_for_game_sim_};
use crate::league::{get_league, okapi_add_operation_for_get_league_};
use crate::league::team::{add_team, okapi_add_operation_for_add_team_};

#[macro_use] extern crate rocket;

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
        .mount("/", routes![game_sim,get_league,add_team])
        .mount("/", catch_all_options_routes())
        .mount("/openapi", openapi_get_routes![game_sim,get_league,add_team])
        .mount("/openapi", make_swagger_ui(&SwaggerUIConfig {
            url: "openapi.json".to_owned(),
            ..Default::default()
        }))
        .attach(cors.clone())
        .manage(cors)
}
