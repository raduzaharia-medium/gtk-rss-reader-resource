mod main_window;

use main_window::MainWindow;
use gtk4::prelude::GtkWindowExt;

use gtk4::{
    gio::resources_register_include,
    prelude::{ApplicationExt, ApplicationExtManual},
};
use libadwaita::Application;

fn build_ui(application: &Application) {
    let window = MainWindow::new(application);
    window.set_image();
    window.present();
}

pub fn main() {
    resources_register_include!("gtk-rss-reader.gresource").expect("Failed to register resources.");

    let application = Application::builder()
        .application_id("com.example.gtk-rss-reader")
        .build();

    application.connect_activate(build_ui);
    application.run();
}
