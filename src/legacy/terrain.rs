use super::grid::{Coord, Grid2D};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerrainType {
    Grass,
    Sand,
    Water,
    Resource,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerrainMap(pub Box<[[u8; 1024]; 1024]>);
impl TerrainMap {
    pub fn passable(&self, position: Coord) -> bool {
        self.get(position) != TerrainType::Resource
    }
}

impl Grid2D<TerrainType> for TerrainMap {
    const W: usize = 1024;
    const H: usize = 1024;

    fn set(&mut self, _position: Coord, _value: TerrainType) {
        todo!()
    }

    fn get(&self, position: Coord) -> TerrainType {
        match self.0.get(position) {
            0..=7 => TerrainType::Water,
            8..=103 => TerrainType::Sand,
            104..=107 => TerrainType::Grass,
            108..=123 => TerrainType::Sand,
            124.. => TerrainType::Resource,
        }
    }
}
