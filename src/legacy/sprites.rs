use bevy::render::{texture::Image, render_resource::{TextureDimension, TextureFormat, Extent3d}};
use super::palette::PALETTE;

pub fn load() -> Vec<Image> {
    let bytes = include_bytes!("../../assets/glob1-sprites.bin");
    let l = bytes.len();
    let mut i = 0;
    let mut n = 0;
    let mut images = Vec::new();
    while i < l {
        // from Glob1 image viewer code, entry 465 seems invalid
        if n == 465 {
            break;
        }
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
        let w = get_usize(&mut i);
        println!("Loading sprite {n}: is {w} x {h} ({x_extra}, {y_extra})");
        let pixel_count = w as usize * h as usize;
        let mut data = Vec::with_capacity(pixel_count * 4);
        // terrain doesn't have transparency
        let has_transparency = n < 192 || n > 355;
        for _y in 0..h {
            for _x in 0..w {
                let entry_id = bytes[i] as usize;
                // assuming entry 0 is transparent
                if has_transparency && entry_id == 0 {
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
            Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            TextureDimension::D2,
            data,
            TextureFormat::Bgra8Unorm
        ));
        n += 1;
    }
    images
}