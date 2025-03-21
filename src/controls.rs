use adw::prelude::*;
use gtk::{Box, Button, Entry, MenuButton, Orientation, Popover, ProgressBar};
use webkit6::{prelude::WebViewExt, Settings};
use crate::{app::{DEFAULT_URL, PROGRESS_BAR_HEIGHT}, handlers::show_browser_info};

#[derive(Clone)]
pub struct Controls {
    pub back: Button,
    pub forward: Button,
    pub refresh: Button,
    pub url_entry: Entry,
    pub progress_bar: ProgressBar,
    pub url_container: Box,
}

pub fn create_webview_and_controls() -> (webkit6::WebView, Controls) {
    let web_view = webkit6::WebView::new();
    web_view.set_hexpand(true);
    web_view.set_vexpand(true);

    let settings = Settings::builder().build();
    settings.set_enable_developer_extras(true);
    web_view.load_uri(DEFAULT_URL);

    let back = create_button("go-previous-symbolic", "Back");
    let forward = create_button("go-next-symbolic", "Forward");
    let refresh = create_button("view-refresh-symbolic", "Refresh");
    let url_entry = create_url_entry();
    let progress_bar = create_progress_bar();

    let url_container = Box::new(Orientation::Vertical, 0);
    url_container.set_hexpand(true);
    url_container.append(&url_entry);
    url_container.append(&progress_bar);

    let controls = Controls {
        back,
        forward,
        refresh,
        url_entry,
        progress_bar,
        url_container,
    };

    crate::handlers::setup_controls_handlers(&web_view, &controls);

    (web_view, controls)
}

pub fn create_controls_box(controls: &Controls) -> Box {
    let controls_box = Box::new(Orientation::Horizontal, 5);
    controls_box.set_margin_top(5);
    controls_box.set_margin_end(5);
    controls_box.set_margin_bottom(5);
    controls_box.set_margin_start(5);
    
    controls_box.append(&controls.back);
    controls_box.append(&controls.forward);
    controls_box.append(&controls.refresh);
    controls_box.append(&controls.url_container);
    
    controls_box
}

pub fn create_menu_button() -> MenuButton {
    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text("Меню")
        .build();

    let popover = Popover::new();
    let menu_box = Box::new(Orientation::Vertical, 5);
    menu_box.set_margin_start(10);
    menu_box.set_margin_end(10);
    menu_box.set_margin_top(10);
    menu_box.set_margin_bottom(10);

    let info_button = Button::with_label("Информация о браузере");
    menu_box.append(&info_button);

    popover.set_child(Some(&menu_box));
    menu_button.set_popover(Some(&popover));

    info_button.connect_clicked(move |_| {
        show_browser_info();
    });

    menu_button
}

fn create_button(icon: &str, tooltip: &str) -> Button {
    Button::builder()
        .icon_name(icon)
        .tooltip_text(tooltip)
        .build()
}

fn create_progress_bar() -> ProgressBar {
    let progress_bar = ProgressBar::new();
    progress_bar.set_visible(false);
    progress_bar.set_height_request(PROGRESS_BAR_HEIGHT);
    progress_bar
}

fn create_url_entry() -> Entry {
    Entry::builder()
        .hexpand(true)
        .placeholder_text("Enter URL")
        .text(DEFAULT_URL)
        .build()
}