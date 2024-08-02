extern crate gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CssProvider, StyleContext, Box as GtkBox, Orientation};
use gtk::gio::prelude::*;
use gtk::gdk::Display;
use std::process::Command;
use std::env;

fn main() {
    let app = Application::new(
        Some("com.example.gtk-rs-peo"),
        Default::default(),
    );

    app.connect_activate(|app| {
        // Create a new window
        let window = ApplicationWindow::new(app);
        window.set_title("Peo");
        window.set_default_size(1980, 1080);

        let vbox = GtkBox::new(Orientation::Vertical, 5);

        let hbox = GtkBox::new(Orientation::Horizontal, 0);

        let provider = CssProvider::new();
        provider.load_from_data(
            b"button { border: none; box-shadow: none; }"
        ).expect("Failed to load CSS");

        let display = Display::default().expect("Error getting default display");
        let screen = display.default_screen();

        StyleContext::add_provider_for_screen(&screen, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let button1 = Button::with_label("File");
        let button2 = Button::with_label("Edit");
        let button3 = Button::with_label("Edit");
        let button4 = Button::with_label("Tools");
        let button5 = Button::with_label("Settings");
        let button6 = Button::with_label("Support peo");

        let buttons = [&button1, &button2, &button3, &button4, &button5, &button6];

        for button in buttons.iter() {
            let context = button.style_context();
            context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }

        button1.connect_clicked(|_| {
            println!("File button was clicked! It doesn't do anything else.");
        });

        button2.connect_clicked(|_| {
            println!("Edit button 1 was clicked! It doesn't do anything else.");
        });

        button3.connect_clicked(|_| {
            println!("Edit button 2 was clicked! It doesn't do anything else.");
        });

        button4.connect_clicked(|_| {
            println!("Tools button was clicked! It doesn't do anything else.");
        });

        button5.connect_clicked(|_| {
            println!("Settings button was clicked! It doesn't do anything else.");
        });

        button6.connect_clicked(|_| {
            // Expand `~` to the full path
            let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
            let binary_path = format!("/usr/local/bin/peo-support");
            
            println!("Support peo button was clicked! Opening app binary at {}", binary_path);

            Command::new(&binary_path)
                .spawn()
                .expect("Failed to open app binary");
        });

        for button in buttons.iter() {
            hbox.pack_start(&**button, false, false, 0); // Use &**button to satisfy the trait bound
        }

        vbox.pack_start(&hbox, false, false, 0);

        window.add(&vbox);

        window.show_all();
    });

    app.run();
}
