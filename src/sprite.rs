use web_sys::CanvasRenderingContext2d;

use crate::constants::{CANVAS, KEYS};

struct Position {
    x: f64,
    y: f64,
}

struct Velocity {
    x: f64,
    y: f64,
}

pub struct Player {
    jumping: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { jumping: false }
    }
}

pub enum SpriteKind {
    Player(Player),
}

pub struct Sprite {
    kind: SpriteKind,
    position: Position,
    width: f64,
    height: f64,
    velocity: Velocity,
    color: String,
}

impl Sprite {
    pub fn new(kind: SpriteKind, x: f64, y: f64, width: f64, height: f64, color: String) -> Self {
        Self {
            kind,
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

        match &mut self.kind {
            SpriteKind::Player(player) => {
                let Ok(keys) = KEYS.lock() else { return };

                if keys.d.pressed {
                    self.velocity.x = 5.0;
                } else if keys.a.pressed {
                    self.velocity.x = -5.0;
                } else {
                    self.velocity.x = 0.0;
                }

                if keys.w.pressed && !player.jumping && self.velocity.y == 0.0 {
                    player.jumping = true;
                    self.velocity.y = -8.0;
                } else if self.velocity.y == 0.0 {
                    player.jumping = false
                };
            }
        }
    }
}
