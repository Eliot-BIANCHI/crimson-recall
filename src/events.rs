use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

use crate::constants::KEYS;

pub fn key_down(event: &Event) {
    let event = event.dyn_ref::<KeyboardEvent>().unwrap();

    let Ok(mut keys) = KEYS.lock() else { return };

    match event.key().to_lowercase().as_str() {
        "a" => keys.a.pressed = true,
        "d" => keys.d.pressed = true,
        "w" | " " => keys.w.pressed = true,
        _ => {}
    }
}

pub fn key_up(event: &Event) {
    let event = event.dyn_ref::<KeyboardEvent>().unwrap();

    let Ok(mut keys) = KEYS.lock() else { return };

    match event.key().to_lowercase().as_str() {
        "a" => keys.a.pressed = false,
        "d" => keys.d.pressed = false,
        "w" | " " => keys.w.pressed = false,
        _ => {}
    }
}
