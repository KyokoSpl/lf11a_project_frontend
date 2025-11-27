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
        // Load CSS from multiple possible locations
        let provider = CssProvider::new();
        let css_paths = vec![
            "assets/style.css",                                      // Development (source directory)
            "style.css",                                             // Current directory (portable)
            "/usr/share/lf11a-project-frontend/style.css",          // Linux system install
        ];
        
        let mut css_loaded = false;
        for path in css_paths {
            if std::path::Path::new(path).exists() {
                provider.load_from_path(path);
                css_loaded = true;
                eprintln!("Loaded CSS from: {}", path);
                break;
            }
        }
        
        if !css_loaded {
            eprintln!("Warning: Could not load CSS file from any location. Using default styles.");
        }
        
        if css_loaded {
            gtk::style_context_add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        
        gui::main_window::build(app);
    });

    app.run();
}
