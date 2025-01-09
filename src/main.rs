use image::{DynamicImage, GrayImage, RgbImage, Rgb};

use procedural::ProceduralGrid;

pub mod procedural;

fn main() {
    let grid = ProceduralGrid::new();
    let mut buf = vec![vec![0; 256]; 256];

    let mut img = RgbImage::new(256, 256);

    let mut min = i32::MAX; // 2147483647
    let mut max = i32::MIN; // -2147483648

    for y in 0..256 {
        for x in 0..256 {
            // let x = x as i64;
            // let y = y as i64;
            // let val = ((x ^ y) % 11) | ((x | y) % 17);
            // let val = (x | y) % 17;
            // let val: i64 = ((((x | 12) - (-x)) ^ (!(x % y))) & (((!y) - (20 +
            // x)) - (!(-y)))) % 9;

            let val = grid.calc(x, y);

            if val < min {
                min = val;
            }

            if val > max {
                max = val;
            }

            // if val > 0 {
            //     img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]))
            // }

            buf[x as usize][y as usize] = val;
        }
    }

    for row in buf.iter_mut() {
        for val in row.iter_mut() {
            *val = ((*val-min) * 255) / (max+(-min));
        }
    }

    for y in 0..256 {
        for x in 0..256 {
            let val = buf[x as usize][y as usize] as u8;

            img.put_pixel(x as u32, y as u32, Rgb([val, val, val]))

        }
    }

    let img = DynamicImage::ImageRgb8(img);
    // let img: GrayImage = .into_luma8();
    // img.resize(256, 256, image::imageops::FilterType::Nearest);

    img.save("grid.png").unwrap();

    println!("{:#?}", grid);
}
