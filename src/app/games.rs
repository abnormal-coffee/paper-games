use eframe::{self, egui::{Context, Ui, RichText}, epaint::Vec2};

mod dandelions;
mod naughts_and_crosses;
mod nim;
mod sprouts;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum Game {
    NaughtsAndCrosses(naughts_and_crosses::GameData),
    Sprouts(),
    Nim(),
    Dandelions(),
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Default, Clone)]
pub enum Games {
    #[default]
    Library,
    Game(Game)
}

trait GameButton {
    fn button(ctx: &Context, game: Game, name: &str, games: &mut Games) {
        eframe::egui::CentralPanel::default().show(ctx, |ui | {
            if ui.add(eframe::egui::Button::new(RichText::new(name).size(32.)).min_size(Vec2 { x: 128., y: 10. })).clicked() {
                *games = Games::Game(game.clone());
            };
        });
    }
}

impl GameButton for Games {}

trait Library {
    fn library(ctx: &Context, games: &mut Games) {
        Games::button(ctx, Game::NaughtsAndCrosses(naughts_and_crosses::GameData::default()), "Naughts And Crosses", games);
    }
}

impl Library for Games {}

impl Games {
    pub fn ui(games: &mut self::Games, ctx: &Context) {
        match games {
            Self::Library => {
                eframe::egui::CentralPanel::default().show(ctx, |ui| {
                    Games::library(ctx, games);
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