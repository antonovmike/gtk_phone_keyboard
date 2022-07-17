use gtk::prelude::*;

mod buttons;

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default());
    // Some("com.github.gtk-rs.examples.grid") - application_id

    application.connect_activate(buttons::build_ui);
    application.run();
}