extern crate image;

use image::{ImageBuffer, RgbImage};

const WIDTH: u32 = 8000;
const HEIGHT: u32 = 6000;
const CX: f64 = -0.7269;
const CY: f64 = 0.1889;

fn scale_pixel(r: f64, c: u32, s: u32) -> f64 {
    (r / s as f64) * c as f64 - (0.5 * r)
}

fn main() {
    let r = 3.0;
    let max_iteration: u32 = 110;

    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut zx = scale_pixel(r, x, WIDTH);
        let mut zy = scale_pixel(r, y, HEIGHT);
        let mut iteration = 0;

        let mut done = false;
        while !done {
            let xtemp = zx * zx - zy * zy;
            zy = 2.0 * zx * zy + CY;
            zx = xtemp + CX;

            iteration += 1;

            if zx * zx + zy * zy > r * r || iteration == max_iteration {
                done = true
            }
        }

        let it =
            iteration as f64 + 2.0 - (zx * zx + zy * zy).log10().log10() / (2.0 as f64).log10();
        let it = (255.0 * (it / (max_iteration as f64))) as u8;

        *pixel = image::Rgb([255 - it, 255 - it, 255 - it])
    }
    img.save("tst.png").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scale_pixel() {
        let r: f64 = 10.0;
        assert_eq!(scale_pixel(r, 512, WIDTH), 5.0);
        assert_eq!(scale_pixel(r, 0, WIDTH), -5.0);
    }
}
