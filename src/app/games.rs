use eframe::{self, egui::{Context, Ui, RichText}, epaint::Vec2};
use egui_extras::RetainedImage;
use tinyrand::StdRand;

mod dandelions;
mod naughts_and_crosses;
mod nim;
mod sprouts;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum Game {
    NaughtsAndCrosses(naughts_and_crosses::GameData),
    Sprouts(),
    Nim(nim::Nim),
    Dandelions(),
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Default, Clone)]
pub enum Games {
    #[default]
    Library,
    Game(Game)
}

fn icon(name: &str) -> RetainedImage {
    match name {
        "Tic Tac Toe" => {return RetainedImage::from_image_bytes("icon.png", include_bytes!("games/assets/naughts-and-crosses-512.png")).unwrap();}
        "Nim" => {return RetainedImage::from_image_bytes("icon.png", include_bytes!("games/assets/naughts-and-crosses-512.png")).unwrap();}
        _ => {return RetainedImage::from_image_bytes("icon.png", include_bytes!("games/assets/unknown.png")).unwrap();}
    }
}

trait GameButton {
    fn button(ctx: &Context, game: Game, name: &str, games: &mut Games, ui: &mut Ui) {
        eframe::egui::Frame::none().show(ui, |ui| {
            if ui.add(eframe::egui::ImageButton::new(icon(name).texture_id(ctx), Vec2{x: 256., y: 256.})).clicked() {
                *games = Games::Game(game.clone());
            }
            if ui.add(eframe::egui::Button::new(RichText::new(name).size(32.)).min_size(Vec2 { x: 256., y: 32. })).clicked() {
                *games = Games::Game(game.clone());
            }
        });
    }
}

impl GameButton for Games {}

trait Library {
    fn library(ctx: &Context, games: &mut Games) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            Games::button(ctx, Game::Nim(nim::Nim::default()), "Nim", games, ui);
            Games::button(ctx, Game::NaughtsAndCrosses(naughts_and_crosses::GameData::default()), "Tic Tac Toe", games, ui);
        });
    }
}

impl Library for Games {}

impl Games {
    pub fn ui(games: &mut self::Games, ctx: &Context, random: &mut StdRand) {
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
                    Game::Nim(data) => {nim::Nim::game(data, ctx, random)}
                    Game::Dandelions() => {}
                }
            }
        }
    }
}