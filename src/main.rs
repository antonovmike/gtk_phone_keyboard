use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use gtk::{Button, Grid};

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default());
    // Some("com.github.gtk-rs.examples.grid") - application_id

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
    let button0: Button = builder.object("button0").expect("Couldn't get button0");
    button0.connect_clicked(glib::clone!(@weak grid => move |button| {
        println!("Button 0");
        let left_attach = grid.cell_left_attach(button);
        let new_left_attach = if left_attach == 2 { 0 } else { left_attach + 1 };
        grid.set_cell_left_attach(button, new_left_attach);
    }));

    // Connect buttons from grid.ui to main.rs and add some function to them
    let button1: Button = builder.object("button1").expect("Couldn't get button1");
    let button2: Button = builder.object("button2").expect("Couldn't get button2");
    let button3: Button = builder.object("button3").expect("Couldn't get button3");
    let button4: Button = builder.object("button4").expect("Couldn't get button4");
    let button5: Button = builder.object("button5").expect("Couldn't get button5");
    let button6: Button = builder.object("button6").expect("Couldn't get button6");
    let button7: Button = builder.object("button7").expect("Couldn't get button7");
    let button8: Button = builder.object("button8").expect("Couldn't get button8");
    let button9: Button = builder.object("button9").expect("Couldn't get button9");

    // Set the label to numerical letter after the button has been clicked on
    button1.connect_clicked( move |button| button.set_label("I") );
    button2.connect_clicked( move |button| button.set_label("II") );
    button3.connect_clicked( move |button| button.set_label("III") );
    button4.connect_clicked( move |button| button.set_label("IV") );
    button5.connect_clicked( move |button| button.set_label("V") );
    button6.connect_clicked( move |button| button.set_label("VI") );
    button7.connect_clicked( move |button| button.set_label("VII") );
    button8.connect_clicked( move |button| button.set_label("VIII") );
    button9.connect_clicked( move |button| button.set_label("IX") );

    window.show_all();
}
