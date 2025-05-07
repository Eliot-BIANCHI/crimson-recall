use components::hit_box::HitBox;
use constants::{canvas::CANVAS, player::PLAYER, weapon::WEAPON};
use gloo::events::{EventListener, EventListenerOptions};
use sprites::{
    Sprite,
    platform::Platform,
    player::Player,
    weapon::{Weapon, WeaponState},
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use events::{key_down, key_up, left_click, right_click};

mod components;
mod constants;
mod events;
mod sprites;

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
    let window = get_window();
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

    let mut player = Player::new(
        40.0,
        20.0,
        PLAYER.sprite().width(),
        PLAYER.sprite().height(),
        Some(HitBox::new(
            PLAYER.sprite().width() + WEAPON.sprite().width(),
            PLAYER.sprite().height(),
        )),
        "blue".to_string(),
    );

    let mut weapon = Weapon::new(
        40.0,
        20.0,
        WEAPON.sprite().width(),
        WEAPON.sprite().height(),
        None,
        "red".to_string(),
    );

    let platforms = vec![
        Platform::new(500.0, 500.0, 300.0, 150.0, None, "orange".to_string()),
        Platform::new(200.0, 200.0, 400.0, 100.0, None, "purple".to_string()),
    ];

    *g.borrow_mut() = Some(Closure::new(move || {
        ctx.clear_rect(0.0, 0.0, CANVAS.width(), CANVAS.height());
        ctx.set_fill_style_str("lightgrey");
        ctx.fill_rect(0.0, 0.0, CANVAS.width(), CANVAS.height());

        for platform in &platforms {
            platform.draw(&ctx);
        }

        player.draw(&ctx);
        player.apply_physics();
        player.resolve_collisions(&platforms);
        player.apply_keys();
        player.apply_clicks(&mut weapon);

        weapon.draw(&ctx);

        match weapon.state() {
            WeaponState::Carried => weapon.follow_player(&player),
            WeaponState::Thrown => {
                weapon.apply_physics();
                weapon.resolve_collisions(&platforms);
            }
            WeaponState::Stuck(_) => {}
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    EventListener::new(&window, "keydown", key_down).forget();
    EventListener::new(&window, "keyup", key_up).forget();

    EventListener::new(&canvas, "click", left_click).forget();

    EventListener::new_with_options(
        &canvas,
        "contextmenu",
        EventListenerOptions::enable_prevent_default(),
        right_click,
    )
    .forget();

    Ok(())
}
