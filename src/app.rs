use eframe;
use serde;
use catppuccin_egui;

use self::games::Games;
use tinyrand::{Rand, StdRand};

mod navbar;
mod games;


#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Tab {
    Home,
    Games(games::Games),
    Help,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    tab: Tab,
    #[serde(skip)]
    random: StdRand,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tab: Tab::Home,
            random: StdRand::default(),
        }
    }
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MACCHIATO);
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for AppState {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }


    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let Self { tab , random} = self;

        navbar::NavBar::display(ctx, tab, random);
                
        match tab {
            Tab::Home => {}
            Tab::Help => {}
            Tab::Games(games) => {games::Games::ui(games, ctx, random)}
        }

    }
}