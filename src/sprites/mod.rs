use platform::Platform;
use web_sys::CanvasRenderingContext2d;

use crate::components::{hit_box::HitBox, position::Position};

pub mod platform;
pub mod player;
pub mod weapon;

pub trait Sprite {
    fn position(&self) -> &Position;
    fn previous_position(&self) -> &Position;
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn collision_box(&self) -> &HitBox;
    fn color(&self) -> &str;

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style_str(&self.color());
        let pos = self.position();
        ctx.fill_rect(pos.x(), pos.y(), self.width(), self.height());
    }

    fn apply_physics(&mut self);

    fn resolve_collisions(&mut self, sprites: &Vec<Platform>);
}
