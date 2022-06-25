extern crate gtk;
use gtk::*;

mod buttons;

fn main() {
    // start GTK
    if gtk::init().is_err() {
        eprintln!("GTK failed");
        return;
    }

    let app = buttons::Application::new();   // UI state

    app.window.show_all();  // make widgets visible
    gtk::main();            // start GTK loop
}
