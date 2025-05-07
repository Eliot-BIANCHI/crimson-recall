pub struct HitBox {
    width: f64,
    height: f64,
}

impl HitBox {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }
}
