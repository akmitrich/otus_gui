use crate::ui;
use gtk::glib::clone;
use gtk::prelude::*;
use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
use std::{collections::HashMap, num::ParseFloatError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RetrieveError {
    #[error("Error at server {0}.")]
    ServerError(String),
    #[error("Error in device dict {0}.")]
    BadDeviceDict(String),
    #[error("Parse json error {0}.")]
    ParseJsonError(#[from] reqwest::Error),
    #[error("Parse float error {0}.")]
    ParseFloatError(#[from] ParseFloatError),
}

pub fn setup(ui_data: &ui::Ui) {
    let on_off = &ui_data.on_off;
    let current = &ui_data.current;
    let voltage = &ui_data.voltage;
    ui_data.send.connect_clicked(
        clone!(@weak on_off, @weak current, @weak voltage => move |_| {
            let on = on_off.is_active();
            match parse_entries(current.buffer().text(), voltage.buffer().text()) {
                Ok((current, voltage)) => {
                    send_to_server(on, current, voltage).unwrap();
                }
                Err(e) => {
                    ui::present_error(e.to_string().as_str())
                }
            }
        }),
    );
    ui_data.retrieve.connect_clicked(
        clone!(@weak on_off, @weak current, @weak voltage => move |_| {
            let (on, current_value, voltage_value) = retrieve_from_server().unwrap();
            on_off.set_active(on);
            current.buffer().set_text(&current_value.to_string());
            voltage.buffer().set_text(&voltage_value.to_string());
        }),
    );
}

fn parse_entries(current: String, voltage: String) -> Result<(f64, f64), ParseFloatError> {
    Ok((current.parse()?, voltage.parse()?))
}

fn send_to_server(on: bool, current: f64, voltage: f64) -> Result<(), String> {
    let client = Client::new();
    let current_string = current.to_string();
    let voltage_string = voltage.to_string();
    let query = vec![
        ("device", "socket"),
        ("state", if on { "on" } else { "off" }),
        ("current", current_string.as_str()),
        ("voltage", voltage_string.as_str()),
    ];
    let resp: Response = client
        .post("http://127.0.0.1:4083/update/R/S")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .query(&query)
        .send()
        .expect("Smart home Server must be available.");
    match resp.status() {
        StatusCode::OK => Ok(()),
        _ => Err(format!(
            "Server error. '{}'.",
            resp.text().unwrap_or_else(|_| "Unknown error.".to_string())
        )),
    }
}

fn retrieve_from_server() -> Result<(bool, f64, f64), RetrieveError> {
    let client = Client::new();
    let resp: Response = client
        .get("http://127.0.0.1:4083/R/S")
        .send()
        .expect("Smart home Server must be available.");
    match resp.status() {
        StatusCode::OK => {
            let device_dict = resp.json::<HashMap<String, String>>()?;
            let device_string = device_dict
                .get("device")
                .ok_or_else(|| device_dict_error("Missing device string."))?;
            if device_string != "socket" {
                return Err(device_dict_error("No 'socket' retrieved."));
            }
            let on_off_string = device_dict
                .get("state")
                .ok_or_else(|| device_dict_error("Missing state."))?;
            let on = match on_off_string.as_str() {
                "on" | "вкл" => Some(true),
                "off" | "выкл" => Some(false),
                _ => None,
            }
            .ok_or_else(|| device_dict_error("Unexpected state"))?;
            let current_string = device_dict
                .get("current")
                .ok_or_else(|| device_dict_error("Missing current."))?;
            let current = current_string.parse::<f64>()?;
            let voltage_string = device_dict
                .get("voltage")
                .ok_or_else(|| device_dict_error("Missing voltage."))?;
            let voltage = voltage_string.parse::<f64>()?;
            Ok((on, current, voltage))
        }
        _ => Err(server_error(resp)),
    }
}

fn server_error(resp: Response) -> RetrieveError {
    RetrieveError::ServerError(format!(
        "Server error. '{}'.",
        resp.text().unwrap_or_else(|_| "Unknown error.".to_string())
    ))
}

fn device_dict_error(msg: &str) -> RetrieveError {
    RetrieveError::BadDeviceDict(msg.to_string())
}
