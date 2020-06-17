use image::{ImageBuffer, RgbImage};
use palette::rgb::Rgb;
use palette::Hsv;

#[derive(Copy, Clone)]
enum ColorOption {
    BlackWhite,
    Hsl,
}

#[derive(Clone, Copy)]
struct Julia {
    width: u32,
    height: u32,
    cx: f64,
    cy: f64,
    r: f64,
    max_iterations: u32,
    filename: &'static str,
    color: ColorOption,
}

impl Default for Julia {
    fn default() -> Self {
        Julia {
            width: 8000,
            height: 6000,
            cx: -0.70176,
            cy: -0.3842,
            r: 3.0,
            max_iterations: 500,
            filename: "julia.png",
            color: ColorOption::Hsl,
        }
    }
}

impl Julia {
    fn scale_pixel(self, c: u32, s: u32) -> f64 {
        (self.r / s as f64) * c as f64 - (0.5 * self.r)
    }

    pub fn calculate_set(self) {
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let mut zx = self.scale_pixel(x, self.width);
            let mut zy = self.scale_pixel(y, self.height);
            let mut i = 0;
            let iteration = loop {
                let xtemp = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + self.cy;
                zx = xtemp + self.cx;
                i += 1;
                if zx * zx + zy * zy > self.r * self.r || i == self.max_iterations {
                    break i;
                }
            };
            let it =
                iteration as f64 + 2.0 - (zx * zx + zy * zy).log10().log10() / (2.0 as f64).log10();
            // Match color based on self.color choice
            match self.color {
                ColorOption::BlackWhite => {
                    let it = (255.0 * (it / (self.max_iterations as f64))) as u8;
                    *pixel = image::Rgb([255 - it, 255 - it, 255 - it])
                }
                ColorOption::Hsl => {
                    let rgb_new: Rgb = Hsv::new((it * 5.0) as f32, 1.0 as f32, 1.0 as f32).into();
                    *pixel = image::Rgb([
                        (255.0 - rgb_new.red * 255.0) as u8,
                        (255.0 - rgb_new.green * 255.0) as u8,
                        (255.0 - rgb_new.blue * 255.0) as u8,
                    ]);
                }
            }
        }
        img.save(self.filename).unwrap();
    }
}

fn main() {
    let julia = Julia {
        ..Default::default()
    };
    julia.calculate_set();
}
