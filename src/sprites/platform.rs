use crate::components::{hit_box::HitBox, position::Position};

use super::Sprite;

pub struct Platform {
    position: Position,
    previous_position: Position,
    width: f64,
    height: f64,
    collision_box: HitBox,
    color: String,
}

impl Platform {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        collision_box: Option<HitBox>,
        color: String,
    ) -> Self {
        Self {
            position: Position::new(x, y),
            previous_position: Position::new(x, y),
            width,
            height,
            collision_box: match collision_box {
                Some(hit_box) => hit_box,
                None => HitBox::new(width, height),
            },
            color,
        }
    }
}

impl Sprite for Platform {
    fn apply_physics(&mut self) {}

    fn resolve_collisions(&mut self, _: &Vec<Platform>) {}

    fn position(&self) -> &Position {
        &self.position
    }

    fn previous_position(&self) -> &Position {
        &self.previous_position
    }

    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn collision_box(&self) -> &HitBox {
        &self.collision_box
    }

    fn color(&self) -> &str {
        &self.color
    }
}
