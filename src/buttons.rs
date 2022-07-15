extern crate gtk;

use crate::buttons::gtk::prelude::BoxExt;
use crate::buttons::gtk::prelude::HeaderBarExt;
use crate::buttons::gtk::prelude::WidgetExt;
use crate::buttons::gtk::prelude::ContainerExt;
use crate::buttons::gtk::prelude::GtkWindowExt;

use gtk::*;

pub struct Application {
    pub window: Window,
    pub header: Header,
}

pub struct Header {
    pub container: HeaderBar,
}

pub struct Content {
    pub container: Box,
}

impl Application {
    pub fn new() -> Application {
        let window = Window::new(WindowType::Toplevel);
        // header bar and it's content
        let header = Header::new();
        let content = Content::new();

        // widget's title
        window.set_titlebar(Some(&header.container));
        // window's title
        window.set_title("Just an app");
        // manager's class
        window.set_wmclass("app-name", "Just an app");
        // app's icon
        Window::set_default_icon_name("icon's name");

        window.add(&content.container);

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
        container.set_title(Some("Just an app"));
        // make buttons active
        container.set_show_close_button(true);
        
        Header {
            container,
            // button_1,
        }
    }
}

impl Content {
    fn new() -> Content {
        let container = Box::new(Orientation::Horizontal, 0);

        let button_1 = Button::with_label("Button №1");
        let button_2 = Button::with_label("Button №2");
        container.pack_start(&button_1, true, false, 0);
        container.pack_start(&button_2, true, false, 0);
        button_1.set_halign(Align::Start);
        button_2.set_halign(Align::Center);

        Content {
            container,
        }
    }
}