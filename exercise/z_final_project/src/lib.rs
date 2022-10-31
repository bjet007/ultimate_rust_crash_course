use std::fs;
use std::str::FromStr;

use image::DynamicImage;

pub trait FromArgumentBuilder: Sized {
    type Err;
    fn init_from_arg(args: Vec<String>) -> Result<Self, Self::Err>;
    fn expected_arguments_count() -> usize;
}

pub trait TransformOperation {
    fn apply(&self, img: DynamicImage) -> DynamicImage;
}

pub trait InputOperation {
    fn apply(&self) -> DynamicImage;
}

pub trait OutputOperation {
    fn apply(&self, img: DynamicImage);
}

pub enum RotationAngle {
    R90,
    R180,
    R270,
}

impl FromStr for RotationAngle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<RotationAngle, Self::Err> {
        match s {
            "90" => Ok(RotationAngle::R90),
            "180" => Ok(RotationAngle::R180),
            "270" => Ok(RotationAngle::R270),
            _ => Err("not supported"),
        }
    }
}

pub struct BlurOperation {
    value: f32,
}

impl FromArgumentBuilder for BlurOperation {
    type Err = &'static str;

    fn init_from_arg(mut args: Vec<String>) -> Result<Self, Self::Err> {
        if args.is_empty() {
            return Err("sigma is missing");
        }
        let sigma_param = args.remove(0);
        match sigma_param.parse() {
            Ok(n) => Ok(BlurOperation { value: n }),
            Err(_) => Err("sigma is not a valid f32"),
        }
    }

    fn expected_arguments_count() -> usize {
        1
    }
}

impl TransformOperation for BlurOperation {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        img.blur(self.value)
    }
}

pub struct BrightenOperation {
    value: i32,
}

impl FromArgumentBuilder for BrightenOperation {
    type Err = &'static str;

    fn init_from_arg(mut args: Vec<String>) -> Result<Self, Self::Err> {
        if args.is_empty() {
            return Err("brightness is missing");
        }
        let brightness_param = args.remove(0);
        match brightness_param.parse() {
            Ok(n) => Ok(BrightenOperation { value: n }),
            Err(_) => Err("brightness is not a valid f32"),
        }
    }
    fn expected_arguments_count() -> usize {
        1
    }
}

impl TransformOperation for BrightenOperation {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        img.brighten(self.value)
    }
}

pub struct InvertOperation {}

impl FromArgumentBuilder for InvertOperation {
    type Err = &'static str;
    fn init_from_arg(_: Vec<String>) -> Result<Self, Self::Err> {
        Ok(InvertOperation {})
    }

    fn expected_arguments_count() -> usize {
        0
    }
}

impl TransformOperation for InvertOperation {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        let mut inverted = img;
        inverted.invert();
        inverted
    }
}

pub struct GrayScaleOperation {}

impl FromArgumentBuilder for GrayScaleOperation {
    type Err = &'static str;
    fn init_from_arg(_: Vec<String>) -> Result<Self, Self::Err> {
        Ok(GrayScaleOperation {})
    }

    fn expected_arguments_count() -> usize {
        0
    }
}

impl TransformOperation for GrayScaleOperation {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        img.grayscale()
    }
}

pub struct CropOperation {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl FromArgumentBuilder for CropOperation {
    type Err = &'static str;
    fn init_from_arg(mut args: Vec<String>) -> Result<Self, Self::Err> {
        if args.len() < 4 {
            return Err("rotation is missing");
        }
        let x_param = args.remove(0);
        let y_param = args.remove(0);
        let width_param = args.remove(0);
        let height_param = args.remove(0);
        let x = match x_param.parse() {
            Ok(n) => n,
            Err(_) => {
                return Err("x is not in i32");
            }
        };
        let y = match y_param.parse() {
            Ok(n) => n,
            Err(_) => {
                return Err("y is not in i32");
            }
        };
        let width = match width_param.parse() {
            Ok(n) => n,
            Err(_) => {
                return Err("width is not in i32");
            }
        };
        let height = match height_param.parse() {
            Ok(n) => n,
            Err(_) => {
                return Err("height is not in i32");
            }
        };
        Ok(CropOperation {
            x,
            y,
            width,
            height,
        })
    }

    fn expected_arguments_count() -> usize {
        4
    }
}

impl TransformOperation for CropOperation {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        img.crop_imm(self.x, self.y, self.width, self.height)
    }
}

pub struct RotateOperation {
    rotation_angle: RotationAngle,
}

impl FromArgumentBuilder for RotateOperation {
    type Err = &'static str;
    fn init_from_arg(mut args: Vec<String>) -> Result<Self, Self::Err> {
        if args.is_empty() {
            return Err("rotation is missing");
        }
        let rotation_param = args.remove(0);
        match rotation_param.parse() {
            Ok(n) => Ok(RotateOperation { rotation_angle: n }),
            Err(_) => Err("rotation is not 90|180|270"),
        }
    }

    fn expected_arguments_count() -> usize {
        1
    }
}

impl TransformOperation for RotateOperation {
    fn apply(&self, img: DynamicImage) -> DynamicImage {
        match self.rotation_angle {
            RotationAngle::R90 => img.rotate90(),
            RotationAngle::R180 => img.rotate180(),
            RotationAngle::R270 => img.rotate270(),
        }
    }
}

pub struct GenerateOperation {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromArgumentBuilder for GenerateOperation {
    type Err = &'static str;
    fn init_from_arg(mut args: Vec<String>) -> Result<Self, Self::Err> {
        if args.is_empty() {
            return Err("r g b parameters are missing");
        }
        let r_param = args.remove(0);
        let g_param = args.remove(0);
        let b_param = args.remove(0);
        let r = match r_param.parse() {
            Ok(n) => n,
            Err(_) => {
                return Err("r is not a u8");
            }
        };
        let g = match g_param.parse() {
            Ok(n) => n,
            Err(_) => {
                return Err("g is not a u8");
            }
        };
        let b = match b_param.parse() {
            Ok(n) => n,
            Err(_) => {
                return Err("b is not a u8");
            }
        };

        Ok(GenerateOperation {
            red: r,
            green: g,
            blue: b,
        })
    }

    fn expected_arguments_count() -> usize {
        3
    }
}

impl InputOperation for GenerateOperation {
    fn apply(&self) -> DynamicImage {
        let width = 800;
        let height = 800;

        let mut imgbuf = image::ImageBuffer::new(width, height);
        // Iterate over the coordinates and pixels of the image
        for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
            // Actually set the pixel. red, green, and blue are u8 values!
            *pixel = image::Rgb([self.red, self.green, self.blue]);
        }
        DynamicImage::from(imgbuf)
    }
}

pub struct FractalOperation {}

impl FromArgumentBuilder for FractalOperation {
    type Err = &'static str;
    fn init_from_arg(_: Vec<String>) -> Result<Self, Self::Err> {
        Ok(FractalOperation {})
    }

    fn expected_arguments_count() -> usize {
        0
    }
}

impl InputOperation for FractalOperation {
    fn apply(&self) -> DynamicImage {
        let width = 800;
        let height = 800;

        let mut imgbuf = image::ImageBuffer::new(width, height);

        let scale_x = 3.0 / width as f32;
        let scale_y = 3.0 / height as f32;

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            // Use red and blue to be a pretty gradient background
            let red = (0.3 * x as f32) as u8;
            let blue = (0.3 * y as f32) as u8;

            // Use green as the fractal foreground (here is the fractal math part)
            let cx = y as f32 * scale_x - 1.5;
            let cy = x as f32 * scale_y - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut green = 0;
            while green < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                green += 1;
            }

            // Actually set the pixel. red, green, and blue are u8 values!
            *pixel = image::Rgb([red, green, blue]);
        }

        DynamicImage::from(imgbuf)
    }
}

pub struct OpenFileOperation {
    infile: String,
}

impl FromArgumentBuilder for OpenFileOperation {
    type Err = &'static str;
    fn init_from_arg(mut args: Vec<String>) -> Result<Self, Self::Err> {
        if args.is_empty() {
            return Err("input parameters are missing");
        }
        let infile = args.remove(0);
        Ok(OpenFileOperation { infile })
    }

    fn expected_arguments_count() -> usize {
        0
    }
}

impl InputOperation for OpenFileOperation {
    fn apply(&self) -> DynamicImage {
        image::open(&self.infile).expect("Failed to open INFILE.")
    }
}

pub struct WriteFileOperation {
    outfile: String,
}

impl FromArgumentBuilder for WriteFileOperation {
    type Err = &'static str;
    fn init_from_arg(mut args: Vec<String>) -> Result<Self, Self::Err> {
        if args.is_empty() {
            return Err("input parameters are missing");
        }
        let outfile = args.remove(0);
        Ok(WriteFileOperation { outfile })
    }

    fn expected_arguments_count() -> usize {
        1
    }
}

impl OutputOperation for WriteFileOperation {
    fn apply(&self, img: DynamicImage) {
        let path = std::path::Path::new(&self.outfile);
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).expect("Can't create directory");
        };
        img.save(path).expect("Can't save image");
    }
}
