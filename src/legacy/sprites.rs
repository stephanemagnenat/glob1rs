use super::palette::PALETTE;
use bevy::render::{
    render_resource::{Extent3d, TextureDimension, TextureFormat},
    texture::Image,
};
use log::debug;

pub fn load() -> Vec<Image> {
    let bytes = include_bytes!("../../assets/glob1-sprites.bin");
    let l = bytes.len();
    let mut i = 0;
    let mut n = 0;
    let mut images = Vec::new();
    while i < l {
        if i + 8 > l {
            panic!("End of file before complete header!")
        }
        let get_usize = |i: &mut usize| {
            let v = (bytes[*i] as u32) << 8 | bytes[*i + 1] as u32;
            *i += 2;
            v
        };
        let y_extra = get_usize(&mut i);
        let x_extra = get_usize(&mut i);
        let h = get_usize(&mut i);
        let mut w = get_usize(&mut i);
        let last_col_pad = if w & 0x1 == 1 {
            w += 1;
            true
        } else {
            false
        };
        debug!(
            "Loading sprite {n}: is {w} x {h} ({x_extra}, {y_extra}){}",
            if last_col_pad { " (padding)" } else { "" }
        );
        let pixel_count = w as usize * h as usize;
        let mut data = Vec::with_capacity(pixel_count * 4);
        // terrain doesn't have transparency
        let has_transparency = !(192..=355).contains(&n);
        for _y in 0..h {
            for x in 0..w {
                let entry_id = bytes[i] as usize;
                let ignore_pixel = x + 1 == w && last_col_pad;
                // assuming entry 0 is transparent
                if ignore_pixel || has_transparency && entry_id == 0 {
                    data.push(0);
                    data.push(0);
                    data.push(0);
                    data.push(0);
                } else {
                    let pixel = &PALETTE[entry_id];
                    data.push(pixel.r);
                    data.push(pixel.g);
                    data.push(pixel.b);
                    data.push(255u8);
                }
                i += 1;
            }
        }
        images.push(Image::new(
            Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb, // Rgba8Unorm doesn't work with tilemap
        ));
        n += 1;
    }
    images
}
