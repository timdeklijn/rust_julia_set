use image::{ImageBuffer, RgbImage};
use palette::rgb::Rgb;
use palette::Hsv;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Julua Set", about = "This is a CLI to generate the julia set")]
struct Opt {
    #[structopt(long, default_value = "800")]
    /// Width of the output image in pixels
    width: u32,

    #[structopt(long, default_value = "600")]
    /// Width of the output image in pixels
    height: u32,

    #[structopt(long, default_value = "-0.4")]
    /// Real part of the julia set
    re: f64,

    #[structopt(long, default_value = "0.6")]
    /// Imaginary part of the julia set
    im: f64,

    #[structopt(long, default_value = "3.0")]
    /// Radiut between points to calculate Julia set over
    r: f64,

    #[structopt(long, default_value = "1000")]
    /// Number of iteration per pixel to caluclate values
    max_it: u32,

    #[structopt(long, default_value = "julia.png")]
    /// Name of the output file
    filename: String,

    #[structopt(long, default_value = "hsv")]
    /// What color scheme to use for the fractal
    color: String,

    #[structopt(long, default_value = "1.0")]
    /// Determines range in final color values, the higher the more color cycles.
    color_scale: f64,
}

/// Choice of color schemes.
#[derive(Copy, Clone, Debug)]
enum ColorOption {
    BlackWhite,
    Hsv,
}

/// Julia set struct
#[derive(Clone, Copy)]
struct Julia {
    /// Width of the image
    width: u32,
    /// Height of the image
    height: u32,
    /// Real part of the set
    cx: f64,
    /// Imaginary part of the set
    cy: f64,
    /// Radius used in calculation
    r: f64,
    /// Number of iterations in calculations
    max_iterations: u32,
    /// Color scheme of the calculated image
    color: ColorOption,
    /// Schale the final color value to increase color cycles
    color_scale: f64,
}

impl Julia {
    /// Scale a pixel value to the radius of the current calculation
    fn scale_pixel(self, c: u32, s: u32) -> f64 {
        (self.r / s as f64) * c as f64 - (0.5 * self.r)
    }

    /// Calculate the number of iteration to get to a Julia value. This is untill max_iterations. Once this is 
    /// finished, use this n-iterations to color an image and write the image to a file named `filename`.
    pub fn calculate_set(self, filename: String) {
        // Create image
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);
        // Loop over pixels and get n-iterations
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
            // Color the pixel
            *pixel = if iteration == self.max_iterations {
                image::Rgb([0u8, 0u8, 0u8])
            } else {
                // Convert iteration to smooth value
                let it =
                    iteration as f64 + 2.0 - (zx * zx + zy * zy).log10().log10() / 2.0f64.log10();
                // Match on color scheme
                match self.color {
                    ColorOption::BlackWhite => {
                        let it = (255.0 * (it / (self.max_iterations as f64))) as u8;
                        image::Rgb([255 - it, 255 - it, 255 - it])
                    }
                    ColorOption::Hsv => {
                        let rgb_new: Rgb = Hsv::new((it * self.color_scale) as f32, 1.0f32, 1.0f32).into();
                        image::Rgb([
                            (255.0 - rgb_new.red * 255.0) as u8,
                            (255.0 - rgb_new.green * 255.0) as u8,
                            (255.0 - rgb_new.blue * 255.0) as u8,
                        ])
                    }
                }
            }
        }
        // Write image to file
        img.save(filename).unwrap();
    }
}

fn main() {
    // Parse command line arguments
    let opt = Opt::from_args();
    // Convert string in opt.color to ColorOption
    let color_option = if opt.color == "hsv" {
        ColorOption::Hsv
    } else {
        ColorOption::BlackWhite
    };
    // Create Julia struct from commandline arguments
    let julia = Julia {
        width: opt.width,
        height: opt.height,
        cx: opt.re,
        cy: opt.im,
        r: opt.r,
        max_iterations: opt.max_it,
        color: color_option,
        color_scale: opt.color_scale
    };
    // Calculate Julia set and write to file
    julia.calculate_set(opt.filename);
}
