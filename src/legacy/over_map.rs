use bevy::prelude::Entity;
use delegate::delegate;

use super::grid::{Coord, Grid2D};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OverMapTile {
    #[default]
    Empty,
    Unit(Entity),
    Building(Entity),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OverMap(pub Box<[[OverMapTile; 1024]; 1024]>);
impl Default for OverMap {
    fn default() -> Self {
        Self(box_array![[OverMapTile::default(); 1024]; 1024])
    }
}
impl_grid2d_delegate!(OverMapTile, OverMap);
