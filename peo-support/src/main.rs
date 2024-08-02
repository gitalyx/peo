use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CssProvider, StyleContext, Box as GtkBox, Orientation, Align};
use gio::prelude::*;
use gdk::Screen;
use glib::Bytes;  // glib for Bytes

use std::env;

fn main() {
    let app = Application::new(
        Some("com.example.gtk-rs-peo-support"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);

        window.set_title("Peo support");

        window.set_default_size(700, 500);

        let hbox = GtkBox::new(Orientation::Horizontal, 5);
        hbox.set_valign(Align::Center);  // Center the box vertically
        hbox.set_halign(Align::Start);  // Align the box to the start horizontally (left)

        let button1 = Button::with_label("Donate");
        button1.set_halign(Align::Start);  // Align the button to the start horizontally (left)
        button1.set_margin_start(20);  // Add margin to the left side of the button
        button1.set_size_request(150, 50);  // Set minimum size of the button (width, height)

        button1.connect_clicked(|_| {
            println!("Button 1 was clicked!");
        });

        let button2 = Button::with_label("Help with Code");
        button2.set_halign(Align::Start);  // Align the button to the start horizontally (left)
        button2.set_margin_start(20);  // Add margin to the left side of the button
        button2.set_size_request(150, 50);  // Set minimum size of the button (width, height)

        button2.connect_clicked(|_| {
            // Open the GitHub URL
            let context = gio::AppLaunchContext::new();
            if let Err(e) = gio::AppInfo::launch_default_for_uri("https://github.com/gitalyx/peo", Some(&context)) {
                eprintln!("Failed to open URL: {}", e);
            }
        });

        hbox.add(&button1);
        hbox.add(&button2);

        window.add(&hbox);

        let home_dir = env::var("HOME").expect("Could not get HOME directory");
        let image_path = format!("{}/peo/peo-support/background.png", home_dir);

        println!("Background image path: {}", image_path);

        let css_provider = CssProvider::new();
        let css_data = format!(
            "
                window {{
                    background-image: url('file://{}');
                    background-size: cover;
                }}
                button {{
                    background-color: #4C566A;
                    color: #ECEFF4;
                    padding: 10px 20px;
                    font-size: 16px;
                }}
            ",
            image_path
        );
        css_provider.load_from_data(&Bytes::from(css_data.as_bytes()));

        StyleContext::add_provider_for_screen(
            &Screen::default().expect("Error initializing GTK CSS provider."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.show_all();
    });

    app.run();
}
