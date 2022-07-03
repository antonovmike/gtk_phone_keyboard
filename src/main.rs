// use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
// use gtk::{Button, Grid};

fn main() {
    let application =
    gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("grid.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    window.show_all();
}