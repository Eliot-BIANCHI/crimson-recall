use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent, MouseEvent};

use crate::constants::controls::{KEYS, MOUSE};

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

pub fn left_click(event: &Event) {
    let event = event.dyn_ref::<MouseEvent>().unwrap();

    if event.button() != 0 {
        return;
    };

    let Ok(mut mouse) = MOUSE.lock() else { return };

    mouse.left.pressed = true;
}

pub fn right_click(event: &Event) {
    let event = event.dyn_ref::<MouseEvent>().unwrap();

    if event.button() != 2 {
        return;
    };

    event.prevent_default();

    let Ok(mut mouse) = MOUSE.lock() else { return };

    mouse.right.pressed = true;
    mouse.right.x = event.client_x() as f64;
    mouse.right.y = event.client_y() as f64;
}
