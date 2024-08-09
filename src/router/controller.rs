use axum::{http::StatusCode, Json};
use enigo::*;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Comand {
    keys: Vec<String>,
    action: String,
}

pub async fn controller(Json(payload): Json<Comand>) -> StatusCode {
    simulate_key(&payload.keys);

    StatusCode::OK
}

fn simulate_key(keys: &Vec<String>) {
    let mut enigo = Enigo::new();

    for key in keys {
        match key.as_str() {
            "shift" => enigo.key_down(Key::Shift),
            "ctrl" => enigo.key_down(Key::Control),
            "alt" => enigo.key_down(Key::Alt),
            "b" => enigo.key_click(Key::Layout('b')),
            _ => (),
        }
    }

    enigo.key_up(Key::Shift);
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Alt);
}