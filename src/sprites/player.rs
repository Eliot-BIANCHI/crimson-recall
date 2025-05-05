pub struct Player {
    jumping: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { jumping: false }
    }

    pub fn is_jumping(&self) -> bool {
        self.jumping
    }

    pub fn set_jumping(&mut self, jumping: bool) {
        self.jumping = jumping;
    }
}
