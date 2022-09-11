use gtk::prelude::*;
use gtk::glib::clone;
use std::num::ParseFloatError;
use crate::ui;

pub fn setup(ui: &ui::Ui) {
    let on_off = &ui.on_off;
    let current = &ui.current;
    let voltage = &ui.voltage;
    ui.send.connect_clicked(clone!(@weak on_off, @weak current, @weak voltage => move |_| {
        let on = on_off.is_active();
        match parse_entries(current.buffer().text(), voltage.buffer().text()) {
            Ok((current, voltage)) => println!("Send to server state={:?}, current={}, voltage={}", on, current, voltage),
            Err(e) => println!("Present an error '{e}'."),
        }            
    }));    
    ui.retrieve.connect_clicked(clone!(@weak on_off, @weak current, @weak voltage => move |_| {
        println!("Block main thread and ask Server for socket");
        on_off.set_active(true);
        current.buffer().set_text(&5.0.to_string());
        voltage.buffer().set_text(&219.5.to_string());
    }));
}

fn parse_entries(current: String, voltage: String) -> Result<(f64, f64), ParseFloatError> {
    Ok((current.parse()?, voltage.parse()?))
}