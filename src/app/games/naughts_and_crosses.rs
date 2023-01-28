use tinyrand::{self, StdRand, RandRange};

use eframe::{egui::{Context, RichText}, epaint::Vec2};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct GameData {
    mode: Mode,
    board: Board,
    game_state: GameState,
    starting_player: StartingPlayer,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
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

impl Player {
    fn swap(self: &self::Player) -> Player {
        match self {
            &Player::Zero => {return Player::Cross;}
            &Player::Cross => {return  Player::Zero;}
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Clone)]
enum Mode {
    TwoPlayer(StartingPlayer),
    PlayerVsAi(AIDifficulty),
}

impl Default for Mode {
    fn default() -> Self {
        Self::TwoPlayer(StartingPlayer::default())
    }
}

#[derive(PartialEq, Serialize, Deserialize, Default, Clone)]
enum StartingPlayer {
    Player(Player),
    #[default]
    Random,
}

#[derive(PartialEq, Serialize, Deserialize, Clone)]
enum AIDifficulty {
    Easy,
    Perfect,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
enum Tile{
    Empty,
    Played(Player),
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
enum GameResult{
    Win(Player),
    Draw,
    #[default]
    Unfinished,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl Tile {
    fn tile_char(tile: self::Tile) -> String {
        match tile {
            Tile::Empty => {return " ".to_string();}
            Tile::Played(player) => {
                match player {
                    Player::Zero => {return "○".to_string();}
                    Player::Cross => {return "✖".to_string();}
                }
            }
        }
    }
}

impl Board {
    fn display_board(board: &mut self::Board, ctx: &Context, player: &mut Player,) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            eframe::egui::Grid::new("some_unique_id").spacing(Vec2{x: 10., y: 10.}).show(ui, |ui| {
                for x in 0..3 {
                    for i in 0..3 {
                        if ui.add(eframe::egui::Button::new(RichText::new(Tile::tile_char(board.board[i + (3*x)])).size(200.)).min_size(Vec2{x: 256., y: 256.})).clicked() {
                            if board.board[i + (3*x)] == Tile::Empty {
                                board.board[i + (3*x)] = Tile::Played(player.clone());
                                *player = player.clone().swap();
                            }
                        }
                    }
                    ui.end_row();
                }
            });
        });
    }
}

trait Game {
    fn setup(game_data: &mut GameData, ctx: &Context) {
        eframe::egui::CentralPanel::default().show(ctx, |ui | {
            ui.radio_value(&mut game_data.mode, self::Mode::TwoPlayer(StartingPlayer::Random), "Two Player Local");
            ui.radio_value(&mut game_data.mode, self::Mode::PlayerVsAi(AIDifficulty::Easy), "Single Player Vs AI");
            match &mut game_data.mode {
                Mode::TwoPlayer(player) => {
                    ui.collapsing("Starting Player", |ui| {
                        ui.radio_value(player, StartingPlayer::Random, "Random Starting Player");
                        ui.radio_value(player, StartingPlayer::Player(Player::Zero), "Naughts Goes First");
                        ui.radio_value(player, StartingPlayer::Player(Player::Cross), "Crosses Goes First");
                    });
                }
                Mode::PlayerVsAi(difficulty) => {
                    ui.collapsing("Difficulty", |ui| {
                        ui.radio_value(difficulty, AIDifficulty::Easy, "Easy AI");
                        ui.radio_value(difficulty, AIDifficulty::Perfect, "Perfect");
                    });
                }
            }
            if ui.button("Start Game").clicked() {
                match game_data.starting_player {
                    StartingPlayer::Random => { let rand = StdRand::default().next_range(0..2); let players = [Player::Zero, Player::Cross]; game_data.game_state = GameState::Started(GameResult::default(), players[rand])}
                    StartingPlayer::Player(player) => {game_data.game_state = GameState::Started(GameResult::default(), player)}
                }
            }
        });
    }
    fn game(game_data: &mut GameData, ctx: &Context) {
        if let GameState::Started(_, player) = &mut game_data.game_state {
            Board::display_board(&mut game_data.board, ctx, player)
            
        }
    }
}

trait Result {
    fn draw(ctx: &Context, game_data: &mut GameData) {
        eframe::egui::Window::new("Result").title_bar(false).collapsible(false).show(ctx, |ui| {
            ui.add(eframe::egui::Label::new(RichText::new("The Game Is A Draw").size(64.)));
            if ui.button(RichText::new("New Game").size(32.)).clicked() {
                *game_data = GameData::default();
            }
        });
    }
    fn win(ctx: &Context, game_data: &mut GameData, winner: Player) {
        eframe::egui::Window::new("Result").title_bar(false).collapsible(false).show(ctx, |ui| {
            match winner {
                Player::Zero => {ui.add(eframe::egui::Label::new(RichText::new("Naughts Win").size(64.)));}
                Player::Cross => {ui.add(eframe::egui::Label::new(RichText::new("Crosses Win").size(64.)));}
            }
            if ui.button(RichText::new("New Game").size(32.)).clicked() {
                *game_data = GameData::default();
            }
        });
    }
}

impl Result for GameResult {}

impl GameData {
    pub fn ui(game_data: &mut GameData, ctx: &Context) {
        if let GameResult::Win(player) = GameResult::check_win(game_data.board.clone()) {
            game_data.game_state = GameState::Started(GameResult::Win(player), player);
        }
        match game_data.game_state {
            GameState::Options => {GameData::setup(game_data, ctx)}
            GameState::Started(GameResult::Draw, _) => {GameResult::draw(ctx, game_data)}
            GameState::Started(GameResult::Win(winner), _) => {GameResult::win(ctx, game_data, winner)}
            GameState::Started(GameResult::Unfinished, _) => {GameData::game(game_data, ctx)}
        }
    }
}

impl Game for GameData {}

trait Round {
    fn check_win(state: self::Board) -> GameResult {
        match state.board {
            [Tile::Played(Player::Zero), Tile::Played(Player::Zero), Tile::Played(Player::Zero), _, _, _, _, _, _] => {return GameResult::Win(Player::Zero);}
            [Tile::Played(Player::Cross), Tile::Played(Player::Cross), Tile::Played(Player::Cross), _, _, _, _, _, _] => {return GameResult::Win(Player::Cross);}
            [_, _, _, Tile::Played(Player::Zero), Tile::Played(Player::Zero), Tile::Played(Player::Zero), _, _, _] => {return GameResult::Win(Player::Zero);}
            [_, _, _,  Tile::Played(Player::Cross), Tile::Played(Player::Cross), Tile::Played(Player::Cross), _, _, _] => {return GameResult::Win(Player::Cross);}
            [_, _, _, _, _, _, Tile::Played(Player::Zero), Tile::Played(Player::Zero), Tile::Played(Player::Zero)] => {return GameResult::Win(Player::Zero);}
            [_, _, _, _, _, _, Tile::Played(Player::Cross), Tile::Played(Player::Cross), Tile::Played(Player::Cross)] => {return GameResult::Win(Player::Cross);}
            [Tile::Played(Player::Zero), _, _, _, Tile::Played(Player::Zero), _, _, _, Tile::Played(Player::Zero), ] => {return GameResult::Win(Player::Zero);}
            [_, _, Tile::Played(Player::Zero), _, Tile::Played(Player::Zero), _, Tile::Played(Player::Zero), _, _,] => {return GameResult::Win(Player::Cross);}
            [Tile::Played(Player::Cross), _, _, _, Tile::Played(Player::Cross), _, _, _, Tile::Played(Player::Cross), ] => {return GameResult::Win(Player::Zero);}
            [_, _, Tile::Played(Player::Cross), _, Tile::Played(Player::Cross), _, Tile::Played(Player::Cross), _, _,] => {return GameResult::Win(Player::Cross);}
            [Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_), Tile::Played(_),] => {return GameResult::Draw;}
            _ => {return GameResult::Unfinished;}
        }
    }
}

impl Round for GameResult {}