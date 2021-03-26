//! # Clock example
//!
//! This example demonstrates how to use `gtk::timeout_add_seconds` to run
//! a periodic task, implementing a clock in this example.

use chrono::Local;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

use std::env::args;

fn current_time() -> String {
    return format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title(Some("Clock Example"));
    window.set_default_size(260, 40);

    let time = current_time();
    let label = Label::new(None);
    label.set_text(&time);

    window.set_child(Some(&label));

    window.show();

    // we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        // we could return gtk::Continue(false) to stop our clock after this tick
        glib::Continue(true)
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
}

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.clock"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(build_ui);

    application.run(&args().collect::<Vec<_>>());
}
