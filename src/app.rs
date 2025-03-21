use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar};
use gtk::{Box, Orientation};
use crate::controls::{self, create_webview_and_controls, Controls};

pub const APP_ID: &str = "io.github.aether";
pub const APP_TITLE: &str = "Aether (Browser)";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_WEBSITE: &str = "https://github.com/everyofflineuser/browser/";
pub const DEFAULT_URL: &str = "https://www.google.com/";
pub const SEARCH_ENGINE: &str = "https://www.google.com/search?q=";
pub const PROGRESS_BAR_HEIGHT: i32 = 2;

pub fn build_ui(app: &Application) {
    let window = create_main_window(app);
    let (web_view, controls) = create_webview_and_controls();
    
    let main_box = Box::new(Orientation::Vertical, 0);
    main_box.append(&create_header_bar(&controls));
    main_box.append(&web_view);
    
    window.set_content(Some(&main_box));
    window.present();
}

fn create_main_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .default_width(1200)
        .default_height(800)
        .build()
}

fn create_header_bar(controls: &Controls) -> HeaderBar {
    let header = HeaderBar::new();
    let controls_box = controls::create_controls_box(controls);
    header.set_title_widget(Some(&controls_box));

    let menu_button = controls::create_menu_button();
    header.pack_end(&menu_button);

    header
}