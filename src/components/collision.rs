use crate::sprites::Sprite;

pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn intersects(sprite: &impl Sprite, other: &impl Sprite) -> bool {
    !(sprite.position().x() + sprite.collision_box().width() <= other.position().x()
        || sprite.position().x() >= other.position().x() + other.collision_box().width()
        || sprite.position().y() + sprite.collision_box().height() <= other.position().y()
        || sprite.position().y() >= other.position().y() + other.collision_box().height())
}

pub fn collision_side(sprite: &impl Sprite, other: &impl Sprite) -> Option<Collision> {
    if sprite.previous_position().x() + sprite.collision_box().width() <= other.position().x() {
        return Some(Collision::Left);
    } else if sprite.previous_position().x() >= other.position().x() + other.collision_box().width()
    {
        return Some(Collision::Right);
    } else if sprite.previous_position().y() + sprite.collision_box().height()
        <= other.position().y()
    {
        return Some(Collision::Top);
    } else if sprite.previous_position().y()
        >= other.position().y() + other.collision_box().height()
    {
        return Some(Collision::Bottom);
    } else {
        return None;
    }
}
