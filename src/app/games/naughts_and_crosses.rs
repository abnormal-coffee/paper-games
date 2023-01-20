use std::default;
use rand::{seq::SliceRandom, rngs::ThreadRng};

use eframe::egui::{Ui, Context};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, PartialEq)]
pub struct GameData {
    mode: Mode,
    board: Board,
    game_state: GameState,
    starting_player: StartingPlayer,
}

#[derive(Serialize, Deserialize, Default, PartialEq)]
enum GameState {
    #[default]
    Options,
    Started(GameResult, Player),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum Player{
    Zero,
    Cross,
}

#[derive(PartialEq, Serialize, Deserialize)]
enum Mode {
    TwoPlayer(StartingPlayer),
    PlayerVsAi(AIDifficulty),
}

impl Default for Mode {
    fn default() -> Self {
        Self::TwoPlayer(StartingPlayer::default())
    }
}

#[derive(PartialEq, Serialize, Deserialize, Default)]
enum StartingPlayer {
    Player(Player),
    #[default]
    Random,
}

#[derive(PartialEq, Serialize, Deserialize)]
enum AIDifficulty {
    Easy,
    Medium,
    Hard,
    Adaptive,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
enum Tile{
    Empty,
    Played(Player),
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
enum GameResult{
    Win(Player),
    Draw,
    #[default]
    Unfinished,
}

type Play = (Player, usize);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Board {
    board: [Tile; 9]
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [Tile::Empty; 9]
        }
    }
}


trait Game {
    fn setup(game_data: &mut GameData, ctx: &Context) {
        eframe::egui::CentralPanel::default().show(ctx, |ui | {
            ui.radio_value(&mut game_data.mode, self::Mode::TwoPlayer(StartingPlayer::Random), "Two Player Local");
            ui.radio_value(&mut game_data.mode, self::Mode::PlayerVsAi(AIDifficulty::Medium), "Single Player Vs AI");
            match &mut game_data.mode {
                Mode::TwoPlayer(player) => {
                    ui.radio_value(player, StartingPlayer::Random, "Random Starting Player");
                    ui.radio_value(player, StartingPlayer::Player(Player::Zero), "Naughts Goes First");
                    ui.radio_value(player, StartingPlayer::Player(Player::Cross), "Crosses Goes First");
                }
                Mode::PlayerVsAi(difficulty) => {
                    ui.collapsing("Difficulty", |ui| {
                        ui.radio_value(difficulty, AIDifficulty::Easy, "Easy AI");
                        ui.radio_value(difficulty, AIDifficulty::Medium, "Medium AI");
                        ui.radio_value(difficulty, AIDifficulty::Hard, "Hard AI");
                        ui.radio_value(difficulty, AIDifficulty::Adaptive, "Adaptive AI");
                    });
                }
            }
            if ui.button("Start Game").clicked() {
                match game_data.starting_player {
                    StartingPlayer::Random => {let mut rng = ThreadRng::default(); game_data.game_state = GameState::Started(GameResult::default(), [Player::Zero, Player::Cross].choose(&mut rng).unwrap().clone())}
                    StartingPlayer::Player(player) => {game_data.game_state = GameState::Started(GameResult::default(), player)}
                }
            }
        });
    }
    fn game(game_data: &mut GameData, ctx: &Context) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                
            });
        });
    }
}

impl GameData {
    pub fn ui(game_data: &mut GameData, ctx: &Context) {
        match game_data.game_state {
            GameState::Options => {GameData::setup(game_data, ctx)}
            GameState::Started(_, _) => {GameData::game(game_data, ctx)}
        }
    }
}

impl Game for GameData {}

trait Round {
    // fn request_play() -> usize {
        // loop {
        //     println!("Please input the index of the box")
        //     let mut input = 0;
        //     io::stdin::().read_line();
        //     if let Ok() = io::stdin::().read_line(){
                
        //     }
        // }
    // }
    // fn play(mut state: &mut self::Board, play: Play) -> Result<Board, String> {
    //     match state[play.1] {
    //         Tile::Empty => {state[play.1] = Tile::Played(play.0); return Ok(state.clone());}
    //         Tile::Played(play) => {Err(format!("'{:?}' is not a valid play", play))}
    //     }
    // }
    fn check_win(state: self::Board) -> GameResult {
        match state {
        //     [Tile::Played(Player::Zero), Tile::Played(Player::Zero), Tile::Played(Player::Zero), _, _, _, _, _, _] => {return GameResult::Win(Player::Zero);}
        //     [Tile::Played(Player::Cross), Tile::Played(Player::Cross), Tile::Played(Player::Cross), _, _, _, _, _, _] => {return GameResult::Win(Player::Cross);}
        //     [_, _, _, Tile::Played(Player::Zero), Tile::Played(Player::Zero), Tile::Played(Player::Zero), _, _, _] => {return GameResult::Win(Player::Zero);}
        //     [_, _, _,  Tile::Played(Player::Cross), Tile::Played(Player::Cross), Tile::Played(Player::Cross), _, _, _] => {return GameResult::Win(Player::Cross);}
        //     [_, _, _, _, _, _, Tile::Played(Player::Zero), Tile::Played(Player::Zero), Tile::Played(Player::Zero)] => {return GameResult::Win(Player::Zero);}
        //     [_, _, _, _, _, _, Tile::Played(Player::Cross), Tile::Played(Player::Cross), Tile::Played(Player::Cross)] => {return GameResult::Win(Player::Cross);}
        //     [Tile::Played(Player::Zero), _, _, _, Tile::Played(Player::Zero), _, _, _, Tile::Played(Player::Zero), ] => {return GameResult::Win(Player::Zero);}
        //     [_, _, Tile::Played(Player::Zero), _, Tile::Played(Player::Zero), _, Tile::Played(Player::Zero), _, _,] => {return GameResult::Win(Player::Cross);}
        //     [Tile::Played(Player::Cross), _, _, _, Tile::Played(Player::Cross), _, _, _, Tile::Played(Player::Cross), ] => {return GameResult::Win(Player::Zero);}
        //     [_, _, Tile::Played(Player::Cross), _, Tile::Played(Player::Cross), _, Tile::Played(Player::Cross), _, _,] => {return GameResult::Win(Player::Cross);}
            // [Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_),] => {return GameResult::Draw;}
            _ => {return GameResult::Unfinished;}
        }
    }
}
impl Round for Board {}

fn game() {
    let mut board: Board = Board::default();
    
    println!("Board: {:?}", board);
}