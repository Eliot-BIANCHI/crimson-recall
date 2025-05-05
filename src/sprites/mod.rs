use player::Player;
use web_sys::CanvasRenderingContext2d;

use crate::{
    components::{collision::Collision, hit_box::HitBox, position::Position, velocity::Velocity},
    constants::{CANVAS, KEYS},
};

pub mod player;

enum SpriteKind {
    Player(Player),
    Platform,
}

pub struct Sprite {
    kind: SpriteKind,
    position: Position,
    previous_position: Position,
    width: f64,
    height: f64,
    collision_box: HitBox,
    velocity: Velocity,
    color: String,
}

impl Sprite {
    pub fn new_player(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        collision_box: Option<HitBox>,
        color: String,
    ) -> Self {
        let player = Player::new();
        let collision_box = match collision_box {
            Some(hit_box) => hit_box,
            None => HitBox::new(x, y, width, height),
        };

        Self {
            kind: SpriteKind::Player(player),
            position: Position::new(x, y),
            previous_position: Position::new(x, y),
            width,
            height,
            collision_box,
            velocity: Velocity::new(0.0, 0.0),
            color,
        }
    }

    pub fn new_platform(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        collision_box: Option<HitBox>,
        color: String,
    ) -> Self {
        let collision_box = match collision_box {
            Some(hit_box) => hit_box,
            None => HitBox::new(x, y, width, height),
        };

        Self {
            kind: SpriteKind::Platform,
            position: Position::new(x, y),
            previous_position: Position::new(x, y),
            width,
            height,
            collision_box,
            velocity: Velocity::new(0.0, 0.0),
            color,
        }
    }
}

impl Sprite {
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style_str(&self.color);
        ctx.fill_rect(
            self.position.x(),
            self.position.y(),
            self.width,
            self.height,
        );
    }

    pub fn update(&mut self, platforms: &Vec<Sprite>) {
        self.previous_position.set_x(self.position.x());
        self.previous_position.set_y(self.position.y());

        self.position.mutate_x(self.velocity.x());
        self.position.mutate_y(self.velocity.y());

        if self.position.y() + self.height + self.velocity.y() < CANVAS.height() {
            self.velocity.mutate_y(CANVAS.gravity());
        } else {
            self.velocity.set_y(0.0);
        }

        for platform in platforms {
            if self.intersects(platform) {
                let collision = self.detect_collision(platform);

                match &mut self.kind {
                    SpriteKind::Player(_) => {
                        if collision.is_horizontal() {
                            self.velocity.set_x(0.0);
                        } else {
                            self.velocity.set_y(0.0);
                        }

                        match collision {
                            Collision::Left => {
                                self.position.set_x(platform.position.x() - self.width);
                            }
                            Collision::Right => {
                                self.position.set_x(platform.position.x() + platform.width);
                            }
                            Collision::Top => {
                                self.position.set_y(platform.position.y() - self.height);
                            }
                            Collision::Bottom => {
                                self.position.set_y(platform.position.y() + platform.height);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        match &mut self.kind {
            SpriteKind::Player(player) => {
                let Ok(keys) = KEYS.lock() else { return };

                if keys.d.pressed {
                    self.velocity.set_x(5.0);
                } else if keys.a.pressed {
                    self.velocity.set_x(-5.0);
                } else {
                    self.velocity.set_x(0.0);
                }

                if keys.w.pressed && !player.is_jumping() && self.velocity.y() == 0.0 {
                    player.set_jumping(true);
                    self.velocity.set_y(-10.0);
                } else if self.velocity.y() == 0.0 {
                    player.set_jumping(false);
                };
            }
            _ => {}
        }
    }
}

impl Sprite {
    fn intersects(&self, other: &Sprite) -> bool {
        !(self.position.x() + self.collision_box.width() <= other.collision_box.x()
            || self.position.x() >= other.collision_box.x() + other.collision_box.width()
            || self.position.y() + self.collision_box.height() <= other.collision_box.y()
            || self.position.y() >= other.collision_box.y() + other.collision_box.height())
    }

    fn detect_collision(&self, other: &Sprite) -> Collision {
        let from_left =
            self.previous_position.x() + self.collision_box.width() <= other.collision_box.x();
        let from_right =
            self.previous_position.x() >= other.collision_box.x() + other.collision_box.width();
        let from_top =
            self.previous_position.y() + self.collision_box.height() <= other.collision_box.y();

        if from_left {
            return Collision::Left;
        } else if from_right {
            return Collision::Right;
        } else if from_top {
            return Collision::Top;
        } else {
            return Collision::Bottom;
        }
    }
}
