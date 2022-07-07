use cgmath::Vector2;

pub type Coord = Vector2<i16>;

pub trait Grid2D<T: Copy> {
    const W: u16;
    const H: u16;

    fn is_in_bounds(position: Coord) -> bool {
        position.x >= 0
            && position.x < Self::W as i16
            && position.y >= 0
            && position.y < Self::H as i16
    }

    fn set(&mut self, position: Coord, value: T);
    fn get(&self, position: Coord) -> T;
}

impl<T: Copy> Grid2D<T> for [[T; 1024]; 1024] {
    const W: u16 = 1024;
    const H: u16 = 1024;

    fn set(&mut self, position: Coord, value: T) {
        self[position.x as usize][position.y as usize] = value;
    }

    fn get(&self, position: Coord) -> T {
        self[position.x as usize][position.y as usize]
    }
}
