use gtk::AboutDialog;
use webkit6::{prelude::*, LoadEvent, WebView};
use crate::{app::{APP_TITLE, APP_VERSION, APP_WEBSITE}, controls::Controls, utils::normalize_url};

pub fn setup_controls_handlers(web_view: &WebView, controls: &Controls) {
    {
        let web_view_clone = web_view.clone();
        controls.refresh.connect_clicked(move |_| {
            web_view_clone.reload();
        });
    }

    {
        let web_view_clone = web_view.clone();
        let controls_clone = controls.clone();
        controls.url_entry.connect_activate(move |entry| {
            let input = entry.text().trim().to_string();
            let url = normalize_url(&input);
            controls_clone.progress_bar.set_fraction(0.0);
            controls_clone.progress_bar.set_visible(true);
            web_view_clone.load_uri(&url);
        });
    }

    {
        let controls_clone = controls.clone();
        web_view.connect_uri_notify(move |view| {
            if let Some(uri) = view.uri() {
                controls_clone.url_entry.set_text(&uri);
            }
        });
    }

    {
        let web_view_clone = web_view.clone();
        controls.back.connect_clicked(move |_| {
            if web_view_clone.can_go_back() {
                web_view_clone.go_back();
            }
        });
    }

    {
        let web_view_clone = web_view.clone();
        controls.forward.connect_clicked(move |_| {
            if web_view_clone.can_go_forward() {
                web_view_clone.go_forward();
            }
        });
    }

    {
        let controls_clone = controls.clone();
        web_view.connect_load_changed(move |_, load_event| {
            match load_event {
                LoadEvent::Started => {
                    controls_clone.progress_bar.set_visible(true);
                    controls_clone.progress_bar.pulse();
                }
                LoadEvent::Finished => {
                    controls_clone.progress_bar.set_visible(false);
                }
                _ => {}
            }
        });
    }
}

pub fn show_browser_info() {
    let about = AboutDialog::builder()
        .program_name(APP_TITLE)
        .version(APP_VERSION)
        .authors(vec!["everyofflineuser (Main Developer)", "Time2138(Tester on MacOS)", "GitHub Contributors ❤️"])
        .license_type(gtk::License::Gpl30)
        .website(APP_WEBSITE)
        .logo_icon_name("open-menu-symbolic")
        .build();

    about.present();
}