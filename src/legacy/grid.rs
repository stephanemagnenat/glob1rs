use cgmath::Vector2;

pub type Coord = Vector2<i16>;

pub trait Grid2D<T: Copy> {
    const W: usize;
    const H: usize;

    fn is_in_bounds(position: Coord) -> bool {
        position.x >= 0 &&
        position.x < 1024 &&
        position.y >= 0 &&
        position.y < 1024
    }

	fn set(&mut self, position: Coord, value: T);
	fn get(&self, position: Coord) -> T;
}

impl<T: Copy> Grid2D<T> for [[T; 1024]; 1024] {
    const W: usize = 1024;
    const H: usize = 1024;

    fn set(&mut self, position: Coord, value: T) {
        self[position.x as usize][position.y as usize] = value;
    }

    fn get(&self, position: Coord) -> T {
        self[position.x as usize][position.y as usize]
    }
}