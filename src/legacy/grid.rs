use cgmath::Vector2;

pub type Coord = Vector2<i16>;

pub trait Grid2D<T: Copy> {
    const W: usize;
    const H: usize;

    fn is_in_bounds(position: Coord) -> bool {
        position.x >= 0
            && position.x < Self::W as i16
            && position.y >= 0
            && position.y < Self::H as i16
    }

    fn for_each(&mut self, f: impl Fn(T, Coord) -> T) {
        for x in 0..Self::W as i16 {
            for y in 0..Self::H as i16 {
                let position = Coord::new(x, y);
                self.set(position, f(self.get(position), position));
            }
        }
    }

    fn set(&mut self, position: Coord, value: T);
    fn get(&self, position: Coord) -> T;
}

impl<T: Copy, const W: usize, const H: usize> Grid2D<T> for [[T; W]; H] {
    const W: usize = W;
    const H: usize = H;

    fn set(&mut self, position: Coord, value: T) {
        self[position.y as usize][position.x as usize] = value;
    }

    fn get(&self, position: Coord) -> T {
        self[position.y as usize][position.x as usize]
    }
}

macro_rules! impl_grid2d_delegate {
    ($tile_ty: ty, $map_ty: ty) => {
        impl Grid2D<$tile_ty> for $map_ty {
            const W: usize = 1024;
            const H: usize = 1024;
            delegate! {
                to self.0 {
                    fn set(&mut self, position: Coord, value: $tile_ty);
                    fn get(&self, position: Coord) -> $tile_ty;
                }
            }
        }
    };
}
