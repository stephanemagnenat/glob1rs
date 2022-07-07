use bevy::prelude::Entity;
use delegate::delegate;

use super::grid::{Grid2D, Coord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverMapTile {
	Empty,
	Unit(Entity)
}
impl Default for OverMapTile {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OverMap(pub Box<[[OverMapTile; 1024]; 1024]>);
impl Default for OverMap {
    fn default() -> Self {
        Self(box_array![[OverMapTile::default(); 1024]; 1024])
    }
}
impl Grid2D<OverMapTile> for OverMap {
	const W: u16 = 1024;
    const H: u16 = 1024;
	delegate! {
        to self.0 {
			fn set(&mut self, position: Coord, value: OverMapTile);
			fn get(&self, position: Coord) -> OverMapTile;
		}
	}
}