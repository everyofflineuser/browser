mod app;
mod controls;
mod handlers;
mod utils;

use adw::prelude::*;
use adw::Application;
use app::APP_ID;
use gtk::glib;

fn main() -> glib::ExitCode {
    adw::init().expect("Failed to initialize libadwaita");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(app::build_ui);
    app.run()
}