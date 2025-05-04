use constants::CANVAS;
use sprite::Sprite;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod constants;
mod sprite;

fn get_window() -> web_sys::Window {
    web_sys::window().expect("No global « window » object")
}

fn get_document() -> web_sys::Document {
    get_window()
        .document()
        .expect("No global « document » object")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    get_window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("« requestAnimationFrame » is not registered");
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let document = get_document();

    let canvas = document
        .get_element_by_id("game")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(CANVAS.width() as u32);
    canvas.set_height(CANVAS.height() as u32);

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut player = Sprite::new(50.0, 100.0, 100.0, 200.0, "blue".to_string());

    *g.borrow_mut() = Some(Closure::new(move || {
        ctx.clear_rect(0.0, 0.0, CANVAS.width(), CANVAS.height());
        ctx.set_fill_style_str("lightgrey");
        ctx.fill_rect(0.0, 0.0, CANVAS.width(), CANVAS.height());

        player.draw(&ctx);
        player.update();

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
