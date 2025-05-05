use super::position::Position;

pub struct HitBox {
    position: Position,
    width: f64,
    height: f64,
}

impl HitBox {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            position: Position::new(x, y),
            width,
            height,
        }
    }

    pub fn x(&self) -> f64 {
        self.position.x()
    }

    pub fn y(&self) -> f64 {
        self.position.y()
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}
