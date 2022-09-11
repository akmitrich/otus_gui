use gtk::prelude::*;

const APP_ID: &str = "org.otus.Socket";
mod logic;
mod ui;

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run();
}
