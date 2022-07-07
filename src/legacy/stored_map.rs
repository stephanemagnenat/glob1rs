use log::{debug, trace};
use rand::{prelude::ThreadRng, Rng};
use std::io::{Error, ErrorKind, Read};

use super::{grid::Coord, terrain::TerrainMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredMap {
    pub terrain: TerrainMap,
    pub queen_positions: Vec<Coord>,
    pub view_position: Coord,
}

/// When a tile is compressed, only its type (e.g. pure water) is stored.
/// This struct allows to sample actual tiles (e.g. pure water third sprite).
struct TerrainTileRange(u8, u8);
impl TerrainTileRange {
    fn decode(encoded: u8) -> Self {
        if encoded < 31 {
            Self(encoded * 4, 4)
        } else {
            Self(4 + (10 * (encoded - 19)), 10)
        }
    }
    fn sample_tile(&self, rng: &mut ThreadRng) -> u8 {
        self.0 + rng.gen_range(0..self.1)
    }
}

pub fn load(input: impl Read) -> Result<StoredMap, Error> {
    let mut bytes = input.bytes();
    debug!("Loading map");
    let mut get_u8 = || {
        bytes
            .next()
            .transpose()
            .and_then(|b| b.ok_or_else(|| Error::from(ErrorKind::UnexpectedEof)))
    };
    let mut map = box_array![[0; 1024]; 1024];
    let mut rng = rand::thread_rng();
    let mut multi_tiles: Option<(u8, i32)> = None;
    for tile_x in 0..64 {
        let base_x = tile_x * 16;
        for tile_y in 0..64 {
            let base_y = tile_y * 16;
            trace!("Processing tile {tile_x} {tile_y}");
            // we are processing a multi-tile encoding
            if let Some((encoded, count)) = &mut multi_tiles {
                trace!("  part of multi-tile encoding");
                // fill one tile
                let tile_range = TerrainTileRange::decode(*encoded);
                for dx in 0..16 {
                    for dy in 0..16 {
                        map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                    }
                }
                // are there tiles left to process?
                if *count == 1 {
                    multi_tiles = None;
                } else {
                    *count -= 1;
                }
                continue;
            }
            // we are processing a new tile
            let compression_type = get_u8()?;
            trace!("  compressed with mode {compression_type}");
            match compression_type {
                // uncompressed
                1 => {
                    for dx in 0..16 {
                        for dy in 0..16 {
                            map[base_x + dx][base_y + dy] = get_u8()?;
                        }
                    }
                }
                // constant tile type
                2 => {
                    let tile_range = TerrainTileRange::decode(get_u8()?);
                    for dx in 0..16 {
                        for dy in 0..16 {
                            map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                        }
                    }
                }
                // linear compression type A
                3 => {
                    let mut filled = 0;
                    while filled < 256 {
                        let tile_range = TerrainTileRange::decode(get_u8()?);
                        let count = get_u8()? as usize;
                        for i in filled..filled + count {
                            let dx = i / 16;
                            let dy = i % 16;
                            map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                        }
                        filled += count;
                    }
                }
                // linear compression type B
                4 => {
                    let mut filled = 0;
                    while filled < 256 {
                        let tile_range = TerrainTileRange::decode(get_u8()?);
                        let count = get_u8()? as usize;
                        for i in filled..filled + count {
                            let dx = i % 16;
                            let dy = i / 16;
                            map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                        }
                        filled += count;
                    }
                }
                // multi-tile constant tile type
                5 => {
                    let encoded = get_u8()?;
                    let count = get_u8()? as i32;
                    trace!("  decompressing {count} tiles with type {encoded}");
                    // fill one tile
                    let tile_range = TerrainTileRange::decode(encoded);
                    for dx in 0..16 {
                        for dy in 0..16 {
                            map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                        }
                    }
                    // schedule the rest
                    if count > 1 {
                        multi_tiles = Some((encoded, count - 1));
                    }
                }
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            }
        }
    }
    // helpers for 16 bit types
    let mut get_u16_be = || {
        let b_high = get_u8();
        b_high.and_then(|b_high| get_u8().map(|b_low| (b_high as u16) << 8 | (b_low as u16)))
    };
    let mut get_coord = || {
        let x = get_u16_be();
        x.and_then(|x| {
            get_u16_be().map(|y| {
                if x != u16::MAX && y != u16::MAX {
                    Some(Coord::new(x as i16, y as i16))
                } else {
                    None
                }
            })
        })
    };
    // load queen positions
    let mut queen_positions = Vec::new();
    for _ in 0..8 {
        if let Some(coord) = get_coord()? {
            queen_positions.push(coord)
        }
    }
    // load view position
    let view_position = get_coord()?.unwrap();
    Ok(StoredMap {
        terrain: TerrainMap(map),
        queen_positions,
        view_position,
    })
}
