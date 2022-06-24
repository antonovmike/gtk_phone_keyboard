extern crate gtk;

use gtk::*;

fn main() {
    // Инициализация GTK.
    if gtk::init().is_err() {
        eprintln!("GTK failed");
        return;
    }

    // UI state
    let app = Application::new();

    // make widgets visible
    app.window.show_all();

    // start GTK loop
    gtk::main();
}

pub struct Application {
    pub window: Window,
    pub header: Header,
}

pub struct Header {
    pub container: HeaderBar,
    pub button_1: Button, // add button on the header
}

impl Application {
    fn new() -> Application {
        let window = Window::new(WindowType::Toplevel);
        // header bar and it's content
        let header = Header::new();

        // widget's title
        window.set_titlebar(&header.container);
        // window's title
        window.set_title("Just an app");
        // manager's class
        window.set_wmclass("simple-gtk", "Just an app");
        // app's icon
        Window::set_default_icon_name("icon's name");

        // Push button = close app
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // return app's state
        Application { window, header }
    }
}

impl Header {
    fn new() -> Header {
        // create widget on the header
        let container = HeaderBar::new();
        // add app's name on the header bar
        container.set_title("Just an app");
        // make buttons active
        container.set_show_close_button(true);

        let button_1 = Button::new_with_label("Button №1"); // DOES NOTHING
        button_1
            .get_style_context()
            .map(|c| c.add_class("no action"));

        button_1.connect_clicked(move |button_1| {
            button_1.set_label("back off!");
        });

        container.pack_start(&button_1);

        // return header's state
        Header {
            container,
            button_1,
        }
    }
}
