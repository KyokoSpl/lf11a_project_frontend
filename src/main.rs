use gtk::prelude::*;
use gtk::{Application, CssProvider, gdk::Display};

mod gui;
mod api;
mod config;

#[tokio::main]
async fn main() {
    // Initialize configuration
    let _ = config::Config::get();
    
    let app = Application::builder()
        .application_id("com.example.personnel")
        .build();

    app.connect_activate(|app| {
        // Load CSS
        let provider = CssProvider::new();
        provider.load_from_path("assets/style.css");
        
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        
        gui::main_window::build(app);
    });

    app.run();
}
