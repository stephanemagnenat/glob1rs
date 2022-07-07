use cgmath::Vector2;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Direction {
    TopLeft = 0,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl Direction {
    pub fn delta(&self) -> Vector2<i16> {
        match self {
            Direction::TopLeft => Vector2::new(-1, -1),
            Direction::Top => Vector2::new(0, -1),
            Direction::TopRight => Vector2::new(1, -1),
            Direction::Right => Vector2::new(1, 0),
            Direction::BottomRight => Vector2::new(1, 1),
            Direction::Bottom => Vector2::new(0, 1),
            Direction::BottomLeft => Vector2::new(-1, 1),
            Direction::Left => Vector2::new(-1, 0),
        }
    }
    pub fn all() -> impl Iterator<Item = Self> {
        (0..8).map(|d| Self::try_from(d).unwrap())
    }
}
