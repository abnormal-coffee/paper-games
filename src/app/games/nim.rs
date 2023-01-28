use eframe::egui::Context;
use serde::{Serialize, Deserialize};
use tinyrand::StdRand;

mod setup;
mod game;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Nim {
    tokens: usize,
    players: (Vec<PlayerType>, usize),
    max: usize,
    stage: Stage,
    ai: game::AI,
}

impl Default for Nim {
    fn default() -> Self {
        return Self {
            tokens: 12,
            players: (vec![PlayerType::Player, PlayerType::Ai], 0),
            max: 2,
            stage: Stage::Setup,
            ai: game::AI::Easy,
        };
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerType {
    Ai,
    Player,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    Setup,
    Game,
    Result,
}

impl Nim {
    pub fn game(game: &mut Nim, ctx: &Context, random: &mut StdRand) {
        match game.stage.clone() {
            Stage::Setup => {
                setup::SetUp::setup(game, ctx)
            }
            Stage::Game => {
                game::game(game, random)
            }
            Stage::Result => {
                
            }
        }
    }
}