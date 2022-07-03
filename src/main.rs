use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use gtk::{Button, Grid};

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

    // button function
    let grid: Grid = builder.object("grid").expect("Couldn't get grid");
    let button6: Button = builder.object("button0").expect("Couldn't get button0");
    button6.connect_clicked(glib::clone!(@weak grid => move |button| {
        let left_attach = grid.cell_left_attach(button);
        let new_left_attach = if left_attach == 2 { 0 } else { left_attach + 1 };
        grid.set_cell_left_attach(button, new_left_attach);
    }));

    window.show_all();
}