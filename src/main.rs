pub mod procedural;
use procedural::ProceduralGrid;
use image::{math, DynamicImage, GrayImage, Rgb, RgbImage};

fn main() {
    let mut img = RgbImage::new(256, 256);
    let mut grid = vec![vec![0;256];256];

    for y in 0..256 {
        for x in 0..256 {
            // let x = x as i64;
            // let y = y as i64;

            // let mut val = (x ^ y) % 9;
            let mut val = ((x | y) - x + y^x) % 17;
            if val > 255    {
                val = 255
            }

            // if val < 0  {
            //     val = 0;
            // }

            let val = val as u8;

            if val > 0  {
                img.put_pixel(x, y, Rgb([255, 255, 255]))
            }

            // println!("{:#?}", val);

            // img.put_pixel(x, y , Rgb([val, val, val]));
        }
    }
    let img = DynamicImage::ImageRgb8(img);
    // img.resize(256, 256, image::imageops::FilterType::Gaussian);
    img.save(r"C:\Antonio Arias\projects\bitgrid\src\hello.png");
    // println!("{grid:?}");
}
