use tinyrand::{self, StdRand, Rand};

use crate::app::games::nim::Nim;

use super::AI;

impl AI {
    pub fn play(game: &mut Nim, random: &mut StdRand) {
        match game.ai {
            Self::Easy => {
                if game.tokens <= game.max {
                    game.tokens = 0;
                }
                else {
                    game.tokens -= random.next_lim_usize(game.tokens);
                    game.players.1 += 1;
                    if game.players.1 >= game.players.0.len() {game.players.1 = 0}
                }
            }
            Self::Medium => {
                
            }
            Self::Hard => {
                
            }
            Self::Adaptive => {
                
            }
        }
    }
}