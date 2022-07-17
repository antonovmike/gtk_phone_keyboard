use gtk::glib;
use glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use gtk::{Button, Grid};
use chrono::Local;

pub fn build_ui(application: &gtk::Application) {
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
    
    // --> ROW 2 COLUMN 4 Quit button
    // !!! Pay attention at object() name
    // in thiscase @weak and @strong are the same
    let quit_button: Button = builder.object("quit_button").expect("Couldn't get quit_button");
    quit_button.connect_clicked(clone!(@weak window => move |_|
        unsafe {
            window.destroy()
        }
    ));

    // Change label
    // type annotations needed
    let counter_label: gtk::Label = builder.object("GtkLabel_1").expect("Couldn't get GtkLabel_1");

    // - + buttons
    let minus_button: Button = builder.object("minus").expect("Couldn't get minus");
    let plus_button: Button = builder.object("plus").expect("Couldn't get plus");

    plus_button.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        let nb = counter_label.text()
            .parse()
            .unwrap_or(0.0);
        counter_label.set_text(&format!("{}", nb + 1.1));
    }));
    minus_button.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        let nb = counter_label.text()
            .parse()
            .unwrap_or(0.0);
        counter_label.set_text(&format!("{}", nb - 1.2));
    }));

    // --> TIMER
    let time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    let label_time: gtk::Label = builder.object("GtkLabel_2").expect("Couldn't get GtkLabel_2");
    label_time.set_text(&time);

    // Changing time
    let tick = move || {
        let time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        label_time.set_text(&time);
        // What is this?
        glib::Continue(true)
    };
    // First digit is the rate of time update in seconds
    glib::timeout_add_seconds_local(1, tick);
    
    window.show_all();
}