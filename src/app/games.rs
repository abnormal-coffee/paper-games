use eframe::{self, egui::Context};

mod dandelions;
mod naughts_and_crosses;
mod nim;
mod sprouts;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Game {
    NaughtsAndCrosses(naughts_and_crosses::GameData),
    Sprouts(),
    Nim(),
    Dandelions(),
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Default)]
pub enum Games {
    #[default]
    Library,
    Game(Game)
}

impl Games {
    pub fn ui(games: &mut self::Games, ctx: &Context) {

        match games {
            Self::Library => {
                eframe::egui::CentralPanel::default().show(ctx, |ui| {
                ui.radio_value(games, Games::Game(Game::NaughtsAndCrosses(naughts_and_crosses::GameData::default())), "Naughts And Crosses");
            });
            }
            Self::Game(game) => {
                match game {
                    Game::NaughtsAndCrosses(data) => {naughts_and_crosses::GameData::ui(data, ctx)}
                    Game::Sprouts() => {}
                    Game::Nim() => {}
                    Game::Dandelions() => {}
                }
            }
        }
    }
}