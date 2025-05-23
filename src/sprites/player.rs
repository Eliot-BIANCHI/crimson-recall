use crate::{
    components::{
        collision::{Collision, collision_side, intersects},
        hit_box::HitBox,
        position::Position,
        velocity::Velocity,
    },
    constants::{
        canvas::CANVAS,
        controls::{KEYS, MOUSE},
        player::PLAYER,
        weapon::WEAPON,
    },
};

use super::{
    Sprite,
    platform::Platform,
    weapon::{StuckOn, Weapon, WeaponState},
};

pub struct Player {
    jumping: bool,
    position: Position,
    previous_position: Position,
    width: f64,
    height: f64,
    collision_box: HitBox,
    velocity: Velocity,
    color: String,
}

impl Player {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        collision_box: Option<HitBox>,
        color: String,
    ) -> Self {
        Self {
            jumping: false,
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

    pub fn apply_keys(&mut self) {
        let Ok(keys) = KEYS.lock() else { return };

        if keys.d.pressed {
            self.velocity.set_x(PLAYER.velocity().x());
        } else if keys.a.pressed {
            self.velocity.set_x(-PLAYER.velocity().x());
        } else {
            self.velocity.set_x(0.0);
        }

        if keys.w.pressed && !self.jumping && self.velocity.y() == 0.0 {
            self.jumping = true;
            self.velocity.set_y(PLAYER.velocity().jump());
        } else if self.velocity.y() == 0.0 {
            self.jumping = false;
        }
    }

    pub fn apply_clicks(&mut self, weapon: &mut Weapon) {
        let Ok(mut mouse) = MOUSE.lock() else { return };

        if !mouse.right.pressed {
            return;
        }

        match weapon.state() {
            WeaponState::Carried => {
                weapon.throw(mouse.right.x, mouse.right.y);
                weapon.set_state(WeaponState::Thrown);
                self.collision_box.set_width(PLAYER.sprite().width());
            }
            WeaponState::Thrown => {}
            WeaponState::Stuck(stuck_on) => {
                self.teleport_to_weapon(weapon, stuck_on);
                weapon.set_state(WeaponState::Carried);
                self.collision_box
                    .set_width(PLAYER.sprite().width() + WEAPON.sprite().width());
            }
        }

        mouse.right.pressed = false;
    }

    fn teleport_to_weapon(&mut self, weapon: &Weapon, stuck_on: &StuckOn) {
        match stuck_on {
            StuckOn::Left => {
                self.position.set_x(weapon.position().x());
                self.position
                    .set_y(weapon.position().y() + weapon.height() - self.height);
            }
            StuckOn::Right => {
                self.position
                    .set_x(weapon.position().x() - self.collision_box.width());
                self.position
                    .set_y(weapon.position().y() + weapon.height() - self.height);
            }
            StuckOn::Top => {
                self.position
                    .set_x(weapon.position().x() - self.collision_box.width());
                self.position.set_y(weapon.position().y());
            }
            StuckOn::Bottom => {
                self.position
                    .set_x(weapon.position().x() - self.collision_box.width());
                self.position
                    .set_y(weapon.position().y() + weapon.height() - self.height);
            }
        }
    }
}

impl Sprite for Player {
    fn apply_physics(&mut self) {
        self.previous_position.set_x(self.position.x());
        self.previous_position.set_y(self.position.y());

        self.position.mutate_x(self.velocity.x());
        self.position.mutate_y(self.velocity.y());

        if self.position.y() + self.height + self.velocity.y() < CANVAS.height() {
            self.velocity.mutate_y(CANVAS.gravity());
        } else {
            self.velocity.set_y(0.0);
        }
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
                }
                Collision::Right => {
                    self.velocity.set_x(0.0);
                    self.position
                        .set_x(platform.position().x() + platform.width());
                }
                Collision::Top => {
                    self.velocity.set_y(0.0);
                    self.position
                        .set_y(platform.position().y() - self.collision_box.height());
                }
                Collision::Bottom => {
                    self.velocity.set_y(0.0);
                    self.position
                        .set_y(platform.position().y() + platform.collision_box().height());
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
