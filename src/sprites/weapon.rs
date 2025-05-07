use crate::{
    components::{
        collision::{Collision, collision_side, intersects},
        hit_box::HitBox,
        position::Position,
        velocity::Velocity,
    },
    constants::{canvas::CANVAS, weapon::WEAPON},
};

use super::{Sprite, platform::Platform, player::Player};

pub enum StuckOn {
    Left,
    Right,
    Top,
    Bottom,
}

pub enum WeaponState {
    Carried,
    Thrown,
    Stuck(StuckOn),
}

pub struct Weapon {
    state: WeaponState,
    angle: f64,
    angular_velocity: f64,
    position: Position,
    previous_position: Position,
    width: f64,
    height: f64,
    collision_box: HitBox,
    velocity: Velocity,
    color: String,
}

impl Weapon {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        collision_box: Option<HitBox>,
        color: String,
    ) -> Self {
        Self {
            state: WeaponState::Carried,
            angle: 0.0,
            angular_velocity: 0.01,
            position: Position::new(x, y),
            previous_position: Position::new(x, y),
            width,
            height,
            collision_box: match collision_box {
                Some(hit_box) => hit_box,
                None => HitBox::new(width, height),
            },
            velocity: Velocity::new(0.0, 0.0),
            color,
        }
    }

    pub fn throw(&mut self, target_x: f64, target_y: f64) {
        let dx = target_x - self.position.x();
        let dy = target_y - self.position.y();

        let angle = dy.atan2(dx);
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        let power = (distance / 10.0).min(20.0);

        self.velocity.set_x(angle.cos() * power);
        self.velocity.set_y(angle.sin() * power);
    }

    pub fn follow_player(&mut self, player: &Player) {
        self.position
            .set_x(player.position().x() + WEAPON.sprite().x_offset());
        self.position
            .set_y(player.position().y() + WEAPON.sprite().y_offset());
    }

    pub fn state(&self) -> &WeaponState {
        &self.state
    }

    pub fn set_state(&mut self, state: WeaponState) {
        self.state = state;
    }
}

impl Sprite for Weapon {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        match self.state {
            WeaponState::Carried | WeaponState::Stuck(_) => {
                ctx.save();

                if ctx
                    .translate(
                        self.position.x() + self.width / 2.0,
                        self.position.y() + self.height / 2.0,
                    )
                    .is_ok()
                    && ctx.rotate(self.angle).is_ok()
                {
                    ctx.set_fill_style_str(&self.color());
                    ctx.fill_rect(
                        -self.width / 2.0,
                        -self.height / 2.0,
                        self.width,
                        self.height,
                    );
                }

                ctx.restore();
            }
            WeaponState::Thrown => {
                ctx.set_fill_style_str(&self.color);
                ctx.fill_rect(
                    self.position.x(),
                    self.position.y(),
                    self.width,
                    self.height,
                );
            }
        }
    }

    fn apply_physics(&mut self) {
        self.previous_position.set_x(self.position.x());
        self.previous_position.set_y(self.position.y());

        self.position.mutate_x(self.velocity.x());
        self.position.mutate_y(self.velocity.y());

        if self.position.y() + self.height + self.velocity.y() < CANVAS.height() {
            self.velocity.mutate_y(CANVAS.gravity());
        } else {
            self.velocity.set_y(0.0);
            self.position.set_y(CANVAS.height() - self.height);
            self.state = WeaponState::Stuck(StuckOn::Bottom);
        }

        self.angle = self.angular_velocity;
    }

    fn resolve_collisions(&mut self, platforms: &Vec<Platform>) {
        for platform in platforms {
            if !intersects(self, platform) {
                continue;
            }

            let Some(collision) = collision_side(self, platform) else {
                continue;
            };

            match collision {
                Collision::Left => {
                    self.velocity.set_x(0.0);
                    self.position
                        .set_x(platform.position().x() - self.collision_box.width());
                    self.state = WeaponState::Stuck(StuckOn::Right);
                }
                Collision::Right => {
                    self.velocity.set_x(0.0);
                    self.position
                        .set_x(platform.position().x() + platform.width());
                    self.state = WeaponState::Stuck(StuckOn::Left);
                }
                Collision::Top => {
                    self.velocity.set_y(0.0);
                    self.position
                        .set_y(platform.position().y() - self.collision_box.height());
                    self.state = WeaponState::Stuck(StuckOn::Bottom);
                }
                Collision::Bottom => {
                    self.velocity.set_y(0.0);
                    self.position
                        .set_y(platform.position().y() + platform.collision_box().height());
                    self.state = WeaponState::Stuck(StuckOn::Top);
                }
            }
        }
    }

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
