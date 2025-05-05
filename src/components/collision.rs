pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

impl Collision {
    pub fn is_horizontal(&self) -> bool {
        match self {
            Collision::Left | Collision::Right => true,
            _ => false,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            Collision::Top | Collision::Bottom => true,
            _ => false,
        }
    }
}
