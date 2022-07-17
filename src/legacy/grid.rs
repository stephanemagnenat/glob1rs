use cgmath::Vector2;
use derive_new::new;

pub type Coord = Vector2<i16>;

pub fn grid_to_world(coord: Coord) -> bevy::prelude::Vec3 {
    bevy::prelude::Vec3::new(
        coord.x as f32 * 32.0,
        -coord.y as f32 * 32.0,
        (coord.y as f32 + 1.0) / 1024.,
    )
}
pub fn grid_to_world_with_delta(coord: Coord, delta: Coord) -> bevy::prelude::Vec3 {
    bevy::prelude::Vec3::new(
        coord.x as f32 * 32.0 + delta.x as f32,
        -(coord.y as f32 * 32.0 + delta.y as f32),
        (coord.y as f32 + 1.0) / 1024.,
    )
}

#[derive(Debug, Clone, Copy, new)]
pub struct Rect {
    pub top_left: Coord,
    pub size: Coord,
}

pub trait Grid2D<T: Copy + PartialEq> {
    const W: usize;
    const H: usize;

    fn is_in_bounds(position: Coord) -> bool {
        position.x >= 0
            && position.x < Self::W as i16
            && position.y >= 0
            && position.y < Self::H as i16
    }

    fn for_each(&mut self, f: impl Fn(T, Coord) -> T) {
        for y in 0..Self::H as i16 {
            for x in 0..Self::W as i16 {
                let position = Coord::new(x, y);
                self.set(position, f(self.get(position), position));
            }
        }
    }

    fn rect_has_value(&self, rect: Rect, value: T) -> bool {
        for y in rect.top_left.y..rect.top_left.y + rect.size.y {
            for x in rect.top_left.x..rect.top_left.x + rect.size.x {
                let position = Coord::new(x, y);
                if self.get(position) != value {
                    return false;
                }
            }
        }
        true
    }
    fn set_rect_value(&mut self, rect: Rect, value: T) -> bool {
        for y in rect.top_left.y..rect.top_left.y + rect.size.y {
            for x in rect.top_left.x..rect.top_left.x + rect.size.x {
                let position = Coord::new(x, y);
                self.set(position, value);
            }
        }
        true
    }

    fn get(&self, position: Coord) -> T;
    fn set(&mut self, position: Coord, value: T);
}

impl<T: Copy + PartialEq, const W: usize, const H: usize> Grid2D<T> for [[T; W]; H] {
    const W: usize = W;
    const H: usize = H;

    fn get(&self, position: Coord) -> T {
        self[position.y as usize][position.x as usize]
    }
    fn set(&mut self, position: Coord, value: T) {
        self[position.y as usize][position.x as usize] = value;
    }
}

macro_rules! impl_grid2d_delegate {
    ($tile_ty: ty, $map_ty: ty) => {
        impl Grid2D<$tile_ty> for $map_ty {
            const W: usize = 1024;
            const H: usize = 1024;
            delegate! {
                to self.0 {
                    fn get(&self, position: Coord) -> $tile_ty;
                    fn set(&mut self, position: Coord, value: $tile_ty);
                }
            }
        }
    };
}
