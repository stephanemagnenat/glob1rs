use delegate::delegate;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::grid::{Coord, Grid2D};

/// A diffusion factor, in proportion/percentage
pub struct DiffusionProportion(u16);
impl DiffusionProportion {
    pub fn from_percent(percent: i32) -> Self {
        assert!(percent >= 0);
        assert!(percent <= 100);
        let value = (percent * 1024) / 100;
        Self(value as u16)
    }
    pub fn from_probability(prob: f32) -> Self {
        assert!(prob >= 0.0);
        assert!(prob <= 1.0);
        let value = prob * 1024.0;
        Self(value as u16)
    }
}

pub trait Dispersion: Grid2D<u16> {
    /// Evaporate by 1
    fn evaporate(&mut self) {
        self.for_each(|intensity, _| intensity.saturating_sub(1))
    }

    /// Diffuse on x when passable, proportion is in percentage between 0 (0%) and 1024 (100%).
    fn diffuse_x(&mut self, proportion: u16, passable: fn(Coord) -> bool) {
        let proportion = proportion as i32;
        let mut deltas = vec![0; Self::W];
        for y in 0..Self::H as i16 {
            // first pass, compute deltas to apply
            for x in 0..(Self::W - 1) {
                let cur_position = Coord::new(x as i16, y);
                let next_position = Coord::new((x + 1) as i16, y);
                if passable(cur_position) && passable(next_position) {
                    let cur_value = self.get(cur_position) as i32;
                    let next_value = self.get(next_position) as i32;
                    let delta = next_value - cur_value;
                    let amount = (delta * proportion) >> 10;
                    deltas[x] += amount;
                    deltas[x + 1] -= amount;
                }
            }
            // second pass, replace values and reset deltas
            #[allow(clippy::needless_range_loop)]
            for x in 0..Self::W {
                let position = Coord::new(x as i16, y);
                let value = self.get(position) as i32;
                self.set(position, (value + deltas[x]) as u16);
                deltas[x] = 0;
            }
        }
    }

    /// Diffuse on y when passable, proportion is in percentage between 0 (0%) and 1024 (100%).
    fn diffuse_y(&mut self, proportion: u16, passable: fn(Coord) -> bool) {
        let proportion = proportion as i32;
        let mut deltas = vec![0; Self::H];
        for x in 0..Self::W as i16 {
            // first pass, compute deltas to apply
            for y in 0..(Self::H - 1) {
                let cur_position = Coord::new(x, y as i16);
                let next_position = Coord::new(x, (y + 1) as i16);
                if passable(cur_position) && passable(next_position) {
                    let cur_value = self.get(cur_position) as i32;
                    let next_value = self.get(next_position) as i32;
                    let delta = next_value - cur_value;
                    let amount = (delta * proportion) >> 10;
                    deltas[y] += amount;
                    deltas[y + 1] -= amount;
                }
            }
            // second pass, replace values and reset deltas
            #[allow(clippy::needless_range_loop)]
            for y in 0..Self::H {
                let position = Coord::new(x, y as i16);
                let value = self.get(position) as i32;
                self.set(position, (value + deltas[y]) as u16);
                deltas[y] = 0;
            }
        }
    }

    /// Diffuse on both x and y when passable, proportion is in percentage between 0 (0%) and 1024 (100%).
    ///
    /// Uses the linearity of diffusion and calls first diffuse_x, then diffuse_y.
    fn diffuse(&mut self, proportion: u16, passable: fn(Coord) -> bool) {
        self.diffuse_x(proportion, passable);
        self.diffuse_y(proportion, passable);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ResourceType {
    Wheat = 0,
    Wood,
    Stone,
    Algae,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PheromoneMap(pub Box<[[u16; 1024]; 1024]>);
impl Default for PheromoneMap {
    fn default() -> Self {
        Self(box_array![[0; 1024]; 1024])
    }
}
impl_grid2d_delegate!(u16, PheromoneMap);
impl Dispersion for PheromoneMap {}

pub struct Team {
    // one per resource, and for each, gather/collect
    pub pheromone_maps: [PheromoneMap; 8],
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    impl<const W: usize, const H: usize> Dispersion for [[u16; W]; H] {}

    #[test]
    fn diffuse_x() {
        // normal
        let mut map = [[4, 8, 0]];
        map.diffuse_x(512, |_| true);
        assert_eq!(map[0], [6, 2, 4]);
        map.diffuse_x(256, |_| true);
        assert_eq!(map[0], [5, 3, 4]);
        // with blocking on the side
        let mut map = [[4, 4, 8, 0, 8]];
        let passable = |coord: Coord| coord.x > 0 && coord.x < 4;
        map.diffuse_x(512, passable);
        assert_eq!(map[0], [4, 6, 2, 4, 8]);
        map.diffuse_x(256, passable);
        assert_eq!(map[0], [4, 5, 3, 4, 8]);
        // with blocking in the middle
        let mut map = [[4, 8, 8, 0, 8]];
        let passable = |coord: Coord| coord.x != 2;
        map.diffuse_x(512, passable);
        assert_eq!(map[0], [6, 6, 8, 4, 4]);
        map.diffuse_x(256, passable);
        assert_eq!(map[0], [6, 6, 8, 4, 4]);
    }

    #[test]
    fn diffuse_y() {
        // normal
        let mut map = [[4], [8], [0]];
        map.diffuse_y(512, |_| true);
        assert_eq!(map[0][0], 6);
        assert_eq!(map[1][0], 2);
        assert_eq!(map[2][0], 4);
        map.diffuse_y(256, |_| true);
        assert_eq!(map[0][0], 5);
        assert_eq!(map[1][0], 3);
        assert_eq!(map[2][0], 4);
        // with blocking on the side
        let mut map = [[4], [4], [8], [0], [8]];
        let passable = |coord: Coord| coord.y > 0 && coord.y < 4;
        map.diffuse_y(512, passable);
        assert_eq!(map[0][0], 4);
        assert_eq!(map[1][0], 6);
        assert_eq!(map[2][0], 2);
        assert_eq!(map[3][0], 4);
        assert_eq!(map[4][0], 8);
        map.diffuse_y(256, passable);
        assert_eq!(map[0][0], 4);
        assert_eq!(map[1][0], 5);
        assert_eq!(map[2][0], 3);
        assert_eq!(map[3][0], 4);
        assert_eq!(map[4][0], 8);
        // with blocking in the middle
        let mut map = [[4], [8], [8], [0], [8]];
        let passable = |coord: Coord| coord.y != 2;
        map.diffuse_y(512, passable);
        assert_eq!(map[0][0], 6);
        assert_eq!(map[1][0], 6);
        assert_eq!(map[2][0], 8);
        assert_eq!(map[3][0], 4);
        assert_eq!(map[4][0], 4);
        map.diffuse_y(256, passable);
        assert_eq!(map[0][0], 6);
        assert_eq!(map[1][0], 6);
        assert_eq!(map[2][0], 8);
        assert_eq!(map[3][0], 4);
        assert_eq!(map[4][0], 4);
    }
}
