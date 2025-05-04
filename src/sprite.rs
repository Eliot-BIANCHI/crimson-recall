use crate::constants::CANVAS;
use web_sys::CanvasRenderingContext2d;

struct Position {
    x: f64,
    y: f64,
}

struct Velocity {
    x: f64,
    y: f64,
}

pub struct Sprite {
    position: Position,
    width: f64,
    height: f64,
    velocity: Velocity,
    color: String,
}

impl Sprite {
    pub fn new(x: f64, y: f64, width: f64, height: f64, color: String) -> Self {
        Self {
            position: Position { x, y },
            width,
            height,
            velocity: Velocity { x: 0.0, y: 0.0 },
            color,
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style_str(&self.color);
        ctx.fill_rect(self.position.x, self.position.y, self.width, self.height);
    }

    pub fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.y + self.height + self.velocity.y < CANVAS.height() {
            self.velocity.y += CANVAS.gravity()
        } else {
            self.velocity.y = 0.0
        }
    }
}
