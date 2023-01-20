use eframe;
use serde;
use catppuccin_egui;

use self::games::Games;


mod navbar;
mod games;


#[derive(serde::Deserialize, serde::Serialize)]
enum Tab {
    Home,
    Games(games::Games),
    Help,
}

enum Game {
    
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    tab: Tab
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tab: Tab::Home
        }
    }
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);
        Default::default()
    }
}

impl eframe::App for AppState {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }


    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let Self { tab } = self;

        *tab = Tab::Games(games::Games::default());
        
        match tab {
            Tab::Home => {}
            Tab::Help => {}
            Tab::Games(games) => {games::Games::ui(games, ctx)}
        }

    }
}