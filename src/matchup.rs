use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use fbsim_core::team::FootballTeam;

/// # `FootballMatchup` struct
///
/// A FootballMatchup represents a matchup between a home and away team
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct FootballMatchup {
    home: FootballTeam,
    away: FootballTeam
}

impl FootballMatchup {
    /// Returns the home team in the matchup
    pub fn home_team(&self) -> &FootballTeam {
        &self.home
    }

    /// Returns the away team in the matchup
    pub fn away_team(&self) -> &FootballTeam {
        &self.away
    }
}
