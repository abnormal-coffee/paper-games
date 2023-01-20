#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;


#[cfg(not(target_arch = "wasm32"))]
fn main() {
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "paper games",
        native_options,
        Box::new(|cc| Box::new(app::AppState::new(cc))),
    );
}


#[cfg(target_arch = "wasm32")]
fn main() {
    
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "paper-games",
            web_options,
            Box::new(|cc| Box::new(app::AppState::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}