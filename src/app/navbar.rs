use eframe::{self, egui::{Context, RichText, Ui}, epaint::{Vec2}};
use catppuccin_egui::{self};
use tinyrand::{self, StdRand, Rand};

use super::{Tab, games::Games};

pub struct NavBar;

mod splash;

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
    pub fn display(ctx: &Context, tab: &mut Tab, random: &mut StdRand) {
        eframe::egui::TopBottomPanel::top("NavBar").min_height(40.).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(eframe::egui::Button::new(RichText::new("Home").size(32.)).min_size(Vec2 { x: 128., y: 10. })).clicked() {
                    *tab = Tab::Home;
                }
                ui.separator();
                if ui.add(eframe::egui::Button::new(RichText::new("Games").size(32.)).min_size(Vec2 { x: 128., y: 10. })).clicked() {
                    *tab = Tab::Games(Games::default());
                }
                ui.separator();
                ui.menu_button(RichText::new("Theme").size(32.), |ui| {
                    NavBar::theme_menu(ctx, ui);
                });
                ui.separator();
                if ui.add(eframe::egui::Button::new(RichText::new("Help").size(32.)).min_size(Vec2 { x: 128., y: 10. })).clicked() {
                    *tab = Tab::Help;
                }
                ui.label(splash::splash(random.next_lim_u32(5)));
                ui.vertical_centered(|ui| {
                    eframe::egui::Frame::dark_canvas(&ctx.style()).show(ui, |ui| {
                        ui.add(eframe::egui::Label::new(RichText::from("Unintrusive banner advert baked into WASM - hard to block with adblock").size(32.).italics()).wrap(false))
                    });                    
                });
            })
        });
    }
}