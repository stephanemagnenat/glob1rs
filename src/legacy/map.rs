use std::io::{Read, Error, ErrorKind};
use rand::{Rng, prelude::ThreadRng};
use log::{trace, debug};

pub type Map = [[u8; 1024]; 1024];

/// When a tile is compressed, only its type (e.g. pure water) is stored.
/// This struct allows to sample actual tiles (e.g. pure water third sprite).
struct TileRange(u8, u8);
impl TileRange {
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

pub fn load(input: impl Read) -> Result<Map, Error> {
    let mut bytes = input.bytes();
    debug!("Loading map");
    let mut get_u8 = ||
        bytes
            .next()
            .transpose()
            .and_then(
                |b| b.ok_or_else(|| Error::from(ErrorKind::UnexpectedEof))
            )
    ;
    let mut map = [[0; 1024]; 1024];
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
                let tile_range = TileRange::decode(*encoded);
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
                },
                // constant tile type
                2 => {
                    let tile_range = TileRange::decode(get_u8()?);
                    for dx in 0..16 {
                        for dy in 0..16 {
                            map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                        }
                    }
                },
                // linear compression type A
                3 => {
                    let mut filled = 0;
                    while filled < 256 {
                        let tile_range = TileRange::decode(get_u8()?);
                        let count = get_u8()? as usize;
                        for i in filled..filled + count {
                            let dx = i / 16;
                            let dy = i % 16;
                            map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                        }
                        filled += count;
                    }
                },
                // linear compression type B
                4 => {
                    let mut filled = 0;
                    while filled < 256 {
                        let tile_range = TileRange::decode(get_u8()?);
                        let count = get_u8()? as usize;
                        for i in filled..filled + count {
                            let dx = i % 16;
                            let dy = i / 16;
                            map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                        }
                        filled += count;
                    }
                },
                // multi-tile constant tile type
                5 => {
                    let encoded = get_u8()?;
                    let count = get_u8()? as i32;
                    trace!("  decompressing {count} tiles with type {encoded}");
                    // fill one tile
                    let tile_range = TileRange::decode(encoded);
                    for dx in 0..16 {
                        for dy in 0..16 {
                            map[base_x + dx][base_y + dy] = tile_range.sample_tile(&mut rng);
                        }
                    }
                    // schedule the rest
                    if count > 1 {
                        multi_tiles = Some((encoded, count - 1));
                    }
                },
                _ => return Err(Error::from(ErrorKind::InvalidData))
            }
        }
    }
    // TODO: load queen and window position
    Ok(map)
}