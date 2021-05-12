use gtk::glib::clone;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.search_bar"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_default_size(400, 400);
    window.set_title(Some("Search Bar"));

    let header_bar = gtk::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let search_button = gtk::ToggleButton::new();
    search_button.set_icon_name("system-search-symbolic");
    header_bar.pack_end(&search_button);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    window.set_child(Some(&container));

    let search_bar = gtk::SearchBar::new();
    container.append(&search_bar);
    search_bar.set_valign(gtk::Align::Start);
    search_bar.set_key_capture_widget(Some(&window));
    search_button
        .bind_property("active", &search_bar, "search-mode-enabled")
        .build();

    let entry = gtk::SearchEntry::new();
    entry.set_hexpand(true);
    search_bar.set_child(Some(&entry));

    let label = gtk::Label::new(Some("Type to start search"));
    label.set_hexpand(true);
    container.append(&label);

    search_button.connect_toggled(clone!(@weak search_bar => move |_| {
        if search_bar.is_search_mode() {
            search_bar.set_search_mode(false);
        } else {
            search_bar.set_search_mode(true);
        };
    }));

    entry.connect_search_started(clone!(@weak search_button => move |_| {
        search_button.set_active(true);
    }));

    entry.connect_stop_search(clone!(@weak search_button => move |_| {
        search_button.set_active(false);
    }));

    entry.connect_search_changed(move |entry| {
        label.set_text(&entry.text());
    });

    window.show();
}
