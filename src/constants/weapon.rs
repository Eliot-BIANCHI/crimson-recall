use once_cell::sync::Lazy;

use super::player::PLAYER;

pub struct Weapon {
    sprite: WeaponSprite,
}

impl Weapon {
    pub fn sprite(&self) -> &WeaponSprite {
        &self.sprite
    }
}

const WEAPON_Y_RATIO: f64 = 1.0 / 5.0;

pub static WEAPON: Lazy<Weapon> = Lazy::new(|| Weapon {
    sprite: WeaponSprite {
        width: 30.0,
        height: 100.0,
        x_offset: PLAYER.sprite().width(),
        y_offset: PLAYER.sprite().height() * WEAPON_Y_RATIO,
    },
});

pub struct WeaponSprite {
    width: f64,
    height: f64,
    x_offset: f64,
    y_offset: f64,
}

impl WeaponSprite {
    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn x_offset(&self) -> f64 {
        self.x_offset
    }

    pub fn y_offset(&self) -> f64 {
        self.y_offset
    }
}
