pub struct Player {
    sprite: PlayerSprite,
    velocity: PlayerVelocity,
}

impl Player {
    pub fn sprite(&self) -> &PlayerSprite {
        &self.sprite
    }

    pub fn velocity(&self) -> &PlayerVelocity {
        &self.velocity
    }
}

pub static PLAYER: Player = Player {
    sprite: PlayerSprite {
        width: 100.0,
        height: 200.0,
    },
    velocity: PlayerVelocity {
        x: 5.0,
        jump: -10.0,
    },
};

pub struct PlayerSprite {
    width: f64,
    height: f64,
}

impl PlayerSprite {
    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}

pub struct PlayerVelocity {
    x: f64,
    jump: f64,
}

impl PlayerVelocity {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn jump(&self) -> f64 {
        self.jump
    }
}
