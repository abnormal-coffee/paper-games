use eframe::{self, egui::{Context, RichText}, epaint::Color32};

use super::{Nim, PlayerType, Stage};

pub enum SetUp {
    Unfinished,
    Finished,
}

impl SetUp {
    pub fn setup(nim: &mut Nim, ctx: &Context) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(eframe::egui::Slider::new(&mut nim.tokens, 8..=200).text(RichText::from("Tokens").size(64.)));
            ui.add(eframe::egui::Slider::new(&mut nim.max, 2..=20).text(RichText::from("Max").size(64.)));
            let (mut remove, mut remove_value) = (false, 0);
            if ui.add(eframe::egui::Button::new(RichText::new("Add Player").size(64.)).min_size(eframe::epaint::Vec2 { x: 512., y: 32. })).clicked() {
                nim.players.0.push(PlayerType::Player)
            }
            if ui.add(eframe::egui::Button::new(RichText::new("Add Ai").size(64.)).min_size(eframe::epaint::Vec2 { x: 512., y: 32. })).clicked() {
                nim.players.0.push(PlayerType::Ai)
            }
            if ui.add(eframe::egui::Button::new(RichText::new("Start Game").size(64.)).min_size(eframe::epaint::Vec2 { x: 512., y: 32. })).clicked() {
                nim.stage = Stage::Game
            }
            ui.horizontal(|ui| {
                for (i, player_type) in nim.players.0.clone().iter().enumerate() {
                    match player_type {
                        &PlayerType::Ai => {
                            if ui.add(eframe::egui::Button::new(RichText::new("⬛").color(Color32::RED).size(64.)).min_size(eframe::epaint::Vec2 { x: 128., y: 128. })).clicked() {
                                (remove, remove_value) = (true, i)
                            }
                        }
                        &PlayerType::Player => {
                            if ui.add(eframe::egui::Button::new(RichText::new("⬛").color(Color32::GREEN).size(64.)).min_size(eframe::epaint::Vec2 { x: 128., y: 128. })).clicked() {
                                (remove, remove_value) = (true, i)
                            }
                        }
                    }
                }
            });
            if remove == true && nim.players.0.len() > 1 {
                nim.players.0.remove(remove_value);
            }
        });
    }
}