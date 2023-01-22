use eframe::{self, egui::{Context, RichText, Ui}, epaint::{TextureId, Vec2}};
use catppuccin_egui::{self, set_theme};

use super::{Tab, games::Games};

pub struct NavBar;

trait SetTheme {
    fn theme_menu(ctx: &Context, ui: &mut Ui) {
        if ui.button(RichText::new("Latte").size(16.)).clicked() {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE)
        }
        if ui.button(RichText::new("Frappe").size(16.)).clicked() {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE)
        }
        if ui.button(RichText::new("Macchiato").size(16.)).clicked() {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO)
        }
        if ui.button(RichText::new("Mocha").size(16.)).clicked() {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA)
        }
        ui.collapsing("Credit", |ui| {
            ui.hyperlink_to("Catppuccin", "https://github.com/catppuccin/catppuccin");
            ui.hyperlink_to("egui-catppuccin", "https://github.com/catppuccin/egui"); 
        });
    }
}

impl SetTheme for NavBar {
    
}

impl NavBar {
    pub fn display(ctx: &Context, tab: &mut Tab) {
        eframe::egui::TopBottomPanel::top("NavBar").min_height(40.).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(eframe::egui::Button::new(RichText::new("Home").size(32.)).min_size(Vec2 { x: 128., y: 10. })).clicked() {
                    *tab = Tab::Home;
                }
                if ui.add(eframe::egui::Button::new(RichText::new("Games").size(32.)).min_size(Vec2 { x: 128., y: 10. })).clicked() {
                    *tab = Tab::Games(Games::default());
                }
                ui.menu_button(RichText::new("Theme").size(32.), |ui| {
                    NavBar::theme_menu(ctx, ui);
                });
                if ui.add(eframe::egui::Button::new(RichText::new("Help").size(32.)).min_size(Vec2 { x: 128., y: 10. })).clicked() {
                    *tab = Tab::Help;
                }
            })
        });
    }
}