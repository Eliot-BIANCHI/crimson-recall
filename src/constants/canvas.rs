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
