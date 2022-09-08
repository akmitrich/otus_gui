use std::num::ParseFloatError;

use gtk::glib::clone;
use gtk::{Application, ApplicationWindow, Button, Label};
use gtk::prelude::*;

const APP_ID: &str = "org.otus.Socket";

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let on_off_label = Label::builder()
        .label("Включить/Выключить")
        .margin_start(10)
        .build();
    let on_off_switch = gtk::Switch::builder()
        .margin_end(10)
        .build();
    let on_off_row = gtk::Box::new(gtk::Orientation::Horizontal, 25);
    on_off_row.append(&on_off_label);
    on_off_row.append(&on_off_switch); 
    let current_label = Label::builder()
        .label("Сила тока:")
        .margin_start(10)
        .build();
    let current_entry = gtk::Entry::builder()
        .margin_end(10)
        .build();
    let current_row = gtk::Box::new(gtk::Orientation::Horizontal, 25);
    current_row.append(&current_label);
    current_row.append(&current_entry);
    let voltage_label = Label::builder()
        .label("Напряжение:")
        .margin_start(10)
        .build();
    let voltage_entry = gtk::Entry::builder()
        .margin_end(10)
        .build();
    let voltage_row = gtk::Box::new(gtk::Orientation::Horizontal, 25);
    voltage_row.append(&voltage_label);
    voltage_row.append(&voltage_entry);
    let send = Button::builder()
        .label("Отправить на Сервер")
        .build();
    send.connect_clicked(clone!(@weak on_off_switch, @weak current_entry, @weak voltage_entry => move |_| {
        let on = on_off_switch.is_active();
        match parse_entries(current_entry.buffer().text(), voltage_entry.buffer().text()) {
            Ok((current, voltage)) => println!("Send to server state={:?}, current={}, voltage={}", on, current, voltage),
            Err(e) => println!("Present an error '{e}'."),
        }
            
    }));
    let retrieve = Button::builder()
        .label("Запросить с Сервера")
        .build();
    retrieve.connect_clicked(clone!(@weak on_off_switch, @weak current_entry, @weak voltage_entry => move |_| {
        println!("Block main thread and ask Server for socket");
        on_off_switch.set_active(true);
        current_entry.buffer().set_text(&5.0.to_string());
        voltage_entry.buffer().set_text(&219.5.to_string());
    }));
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.append(&on_off_row);
    main_box.append(&current_row);
    main_box.append(&voltage_row);
    main_box.append(&send);
    main_box.append(&retrieve);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Otus Socket APP")
        .child(&main_box)
        .build();
    window.present();
}

fn parse_entries(current: String, voltage: String) -> Result<(f64, f64), ParseFloatError> {
    Ok((current.parse()?, voltage.parse()?))
}