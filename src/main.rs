use gtk::prelude::*;

mod buttons;

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default());

    application.connect_activate(buttons::build_ui);
    application.run();
}