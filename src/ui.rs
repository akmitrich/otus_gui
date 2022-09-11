use gtk::prelude::*;
use crate::logic;

pub fn on_off(label: &str) -> (gtk::Box, gtk::Switch) {
    let on_off_label = gtk::Label::builder()
        .label(label)
        .margin_start(10)
        .build();
    let on_off_switch = gtk::Switch::builder()
        .margin_end(10)
        .build();
    let on_off_row = gtk::Box::new(gtk::Orientation::Horizontal, 25);
    on_off_row.append(&on_off_label);
    on_off_row.append(&on_off_switch);
    (on_off_row, on_off_switch)
}

pub fn make_entry(label: &str) -> (gtk::Box, gtk::Entry) {
    let result_label = gtk::Label::builder()
        .label(label)
        .margin_start(10)
        .build();
    let result_entry = gtk::Entry::builder()
        .margin_end(10)
        .build();
    let result_row = gtk::Box::new(gtk::Orientation::Horizontal, 25);
    result_row.append(&result_label);
    result_row.append(&result_entry);
    (result_row, result_entry)
}

pub fn setup_ui() -> (gtk::Box, Ui) {
    let (on_off_row, on_off_switch) = on_off("Вкл/Выкл:");
    let (current_row, current_entry) = make_entry("Сила тока:");
    let (voltage_row, voltage_entry) = make_entry("Напряжение:");
    let send = gtk::Button::builder()
        .label("Отправить на Сервер")
        .build();
    let retrieve = gtk::Button::builder()
        .label("Запросить с Сервера")
        .build();
    let result_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    result_box.set_margin_top(20);
    result_box.set_margin_bottom(20);
    result_box.append(&on_off_row);
    result_box.append(&current_row);
    result_box.append(&voltage_row);
    result_box.append(&send);
    result_box.append(&retrieve);
    (result_box, Ui {on_off: on_off_switch, current: current_entry, voltage: voltage_entry, send, retrieve})
}

pub fn build_ui(app: &gtk::Application) {
    let (main_box, ui) = setup_ui();
    logic::setup(&ui);
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Otus Socket APP")
        .child(&main_box)
        .build();
    window.present();
}

pub struct Ui {
    pub on_off: gtk::Switch,
    pub current: gtk::Entry,
    pub voltage: gtk::Entry,
    pub send: gtk::Button,
    pub retrieve: gtk::Button,
}