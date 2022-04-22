#[repr(C)]
pub struct PaletteEntry {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub type Palette = [PaletteEntry; 256];

lazy_static! {
    /// MacOS classic palette
    pub static ref PALETTE: Palette = {
        use std::mem;
        let bytes = include_bytes!("../../assets/mac-palette.bin");
        assert_eq!(mem::size_of::<Palette>(), bytes.len());
        unsafe { mem::transmute(*bytes) }
    };
}