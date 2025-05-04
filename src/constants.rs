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

pub struct Canvas {
    width: f64,
    height: f64,
    gravity: f64,
}

impl Canvas {
    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn gravity(&self) -> f64 {
        self.gravity
    }
}

pub static CANVAS: Canvas = Canvas {
    width: 1024.0,
    height: 576.0,
    gravity: 0.5,
};
