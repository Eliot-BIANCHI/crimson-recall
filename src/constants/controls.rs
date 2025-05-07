use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Keys {
    pub d: KeyState,
    pub a: KeyState,
    pub w: KeyState,
}

pub struct KeyState {
    pub pressed: bool,
}

pub static KEYS: Lazy<Mutex<Keys>> = Lazy::new(|| {
    Mutex::new(Keys {
        d: KeyState { pressed: false },
        a: KeyState { pressed: false },
        w: KeyState { pressed: false },
    })
});

pub struct Mouse {
    pub left: MouseState,
    pub right: MouseState,
}

pub struct MouseState {
    pub pressed: bool,
    pub x: f64,
    pub y: f64,
}

pub static MOUSE: Lazy<Mutex<Mouse>> = Lazy::new(|| {
    Mutex::new(Mouse {
        left: MouseState {
            pressed: false,
            x: 0.0,
            y: 0.0,
        },
        right: MouseState {
            pressed: false,
            x: 0.0,
            y: 0.0,
        },
    })
});
