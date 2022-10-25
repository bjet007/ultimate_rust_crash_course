// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use std::str::FromStr;

enum RotationAngle {
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
            _ => Err("not supported")
        }
    }
}


fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let sigma_param = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            let sigma = match sigma_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("sigma is not in valid");
                    print_usage_and_exit();
                    return;
                }
            };

            blur(infile, outfile, sigma);
        }
        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let brightness_param = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // **OPTION**
            // Brighten -- see the brighten() function below
            let brightness = match brightness_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("brightness is not in i32");
                    print_usage_and_exit();
                    return;
                }
            };
            brighten(infile, outfile, brightness);
        }
        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let x_param = args.remove(0);
            let y_param = args.remove(0);
            let width_param = args.remove(0);
            let height_param = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x = match x_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("x is not in i32");
                    print_usage_and_exit();
                    return;
                }
            };
            let y = match y_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("y is not in i32");
                    print_usage_and_exit();
                    return;
                }
            };
            let height = match width_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("width is not in i32");
                    print_usage_and_exit();
                    return;
                }
            };
            let width = match height_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("height is not in i32");
                    print_usage_and_exit();
                    return;
                }
            };

            crop(infile, outfile, x, y, width, height);
        }
        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let rotate_param = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // **OPTION**
            // Brighten -- see the brighten() function below
            let rotation_angle = match rotate_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("rotation angle is not valid");
                    print_usage_and_exit();
                    return;
                }
            };

            rotate(infile, outfile, rotation_angle);
        }
        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        }
        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }
        "generate" => {
            if args.len() != 4 {
                print_usage_and_exit();
            }
            let r_param = args.remove(0);
            let g_param = args.remove(0);
            let b_param = args.remove(0);
            let outfile = args.remove(0);

            let r = match r_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("r is not in u8");
                    print_usage_and_exit();
                    return;
                }
            };
            let g = match g_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("r is not in u8");
                    print_usage_and_exit();
                    return;
                }
            };
            let b = match b_param.parse() {
                Ok(n) => {
                    n
                }
                Err(_) => {
                    eprintln!("r is not in u8");
                    print_usage_and_exit();
                    return;
                }
            };
            generate(outfile,r,g,b);
        }


        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur value INFILE OUTFILE");
    println!("brighten value INFILE OUTFILE");
    println!("crop x y width height INFILE OUTFILE");
    println!("rotate 90|180|270 INFILE OUTFILE");
    println!("invert INFILE OUTFILE");
    println!("grayscale INFILE OUTFILE");
    println!("fractal OUTFILE");
    println!("generate r g b OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, sigma: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(sigma);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, brightness: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.brighten(brightness);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    // See blur() for an example of how to open an image.

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.

    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.crop_imm(x, y, width, height);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, rotation_angle: RotationAngle) {
    // See blur() for an example of how to open an image.
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = match rotation_angle {
        RotationAngle::R90 => img.rotate90(),
        RotationAngle::R180 => img.rotate180(),
        RotationAngle::R270 => img.rotate270(),
    };
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.

    let mut img = image::open(infile).expect("Failed to open INFILE.");

    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.
    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, red: u8, green: u8, blue: u8) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    // Iterate over the coordinates and pixels of the image
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
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

    imgbuf.save(outfile).unwrap();
}


// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
