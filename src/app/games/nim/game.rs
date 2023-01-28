use serde::{Serialize, Deserialize};
use tinyrand::StdRand;

use super::{Nim, PlayerType};

mod ai;

#[derive(PartialEq, Default, Serialize, Deserialize, Clone)]
pub enum AI {
    #[default]
    Easy,
    Medium,
    Hard,
    Adaptive,
}

pub fn game(game: &mut Nim, random: &mut StdRand) {
    if game.players.0[game.players.1].clone() == PlayerType::Ai {
        turn(game.players.0[game.players.1], game, random)
    }
}

fn turn(player: PlayerType, game: &mut Nim, random: &mut StdRand) {
    match player {
        PlayerType::Player => {
            
        }
        PlayerType::Ai => {
            AI::play(game, random);
        }
    }
}