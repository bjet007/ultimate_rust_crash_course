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

use mirage::{
    BlurOperation, BrightenOperation, CropOperation, FractalOperation, FromArgumentBuilder,
    GenerateOperation, GrayScaleOperation, InputOperation, InvertOperation, OpenFileOperation,
    OutputOperation, RotateOperation, TransformOperation, WriteFileOperation,
};

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
    let mut input_operation: Option<Box<dyn InputOperation>> = None;
    let mut output_operation: Option<Box<dyn OutputOperation>> = None;
    let mut transforms: Vec<Box<dyn TransformOperation>> = Vec::new();
    loop {
        if args.is_empty() {
            break;
        }

        let subcommand = args.remove(0);
        match subcommand.as_str() {
            // EXAMPLE FOR CONVERSION OPERATIONS
            "blur" => {
                let final_length =
                    std::cmp::min(args.len(), BlurOperation::expected_arguments_count());
                let ops_args = args.drain(..final_length).collect();
                match BlurOperation::init_from_arg(ops_args) {
                    Ok(ops) => transforms.push(Box::new(ops)),
                    Err(invalid_args) => {
                        eprintln!("error: {invalid_args}");
                        print_usage_and_exit();
                    }
                };
            }
            "brighten" => {
                let final_length =
                    std::cmp::min(args.len(), BrightenOperation::expected_arguments_count());
                let ops_args = args.drain(..final_length).collect();
                match BrightenOperation::init_from_arg(ops_args) {
                    Ok(ops) => transforms.push(Box::new(ops)),
                    Err(invalid_args) => {
                        eprintln!("error: {invalid_args}");
                        print_usage_and_exit();
                    }
                };
            }
            "crop" => {
                let final_length =
                    std::cmp::min(args.len(), CropOperation::expected_arguments_count());
                let ops_args = args.drain(..final_length).collect();
                match CropOperation::init_from_arg(ops_args) {
                    Ok(ops) => transforms.push(Box::new(ops)),
                    Err(invalid_args) => {
                        eprintln!("error: {invalid_args}");
                        print_usage_and_exit();
                    }
                };
            }
            "rotate" => {
                let final_length =
                    std::cmp::min(args.len(), RotateOperation::expected_arguments_count());
                let ops_args = args.drain(..final_length).collect();
                match RotateOperation::init_from_arg(ops_args) {
                    Ok(ops) => transforms.push(Box::new(ops)),
                    Err(invalid_args) => {
                        eprintln!("error: {invalid_args}");
                        print_usage_and_exit();
                    }
                };
            }
            "invert" => {
                let final_length =
                    std::cmp::min(args.len(), InvertOperation::expected_arguments_count());
                let ops_args = args.drain(..final_length).collect();
                match InvertOperation::init_from_arg(ops_args) {
                    Ok(ops) => transforms.push(Box::new(ops)),
                    Err(invalid_args) => {
                        eprintln!("error: {invalid_args}");
                        print_usage_and_exit();
                    }
                };
            }
            "grayscale" => {
                let final_length =
                    std::cmp::min(args.len(), GrayScaleOperation::expected_arguments_count());
                let ops_args = args.drain(..final_length).collect();
                match GrayScaleOperation::init_from_arg(ops_args) {
                    Ok(ops) => transforms.push(Box::new(ops)),
                    Err(invalid_args) => {
                        eprintln!("error: {invalid_args}");
                        print_usage_and_exit();
                    }
                };
            }

            // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
            "fractal" => {
                if input_operation.is_some() {
                    eprintln!("error: can't have more than one input operation");
                    print_usage_and_exit();
                }
                let final_length =
                    std::cmp::min(args.len(), FractalOperation::expected_arguments_count());
                let ops_args = args.drain(..final_length).collect();
                match FractalOperation::init_from_arg(ops_args) {
                    Ok(ops) => input_operation = Some(Box::new(ops)),
                    Err(invalid_args) => {
                        eprintln!("error: {invalid_args}");
                        print_usage_and_exit();
                    }
                };
            }
            "generate" => {
                if input_operation.is_some() {
                    eprintln!("error: can't have more than one input operation");
                    print_usage_and_exit();
                }
                let final_length =
                    std::cmp::min(args.len(), GenerateOperation::expected_arguments_count());
                let ops_args = args.drain(..final_length).collect();
                match GenerateOperation::init_from_arg(ops_args) {
                    Ok(ops) => input_operation = Some(Box::new(ops)),
                    Err(invalid_args) => {
                        eprintln!("error: {invalid_args}");
                        print_usage_and_exit();
                    }
                };
            }
            // For everything else...
            other => {
                if input_operation.is_none() {
                    let ops_args = vec![other.to_string()];
                    match OpenFileOperation::init_from_arg(ops_args) {
                        Ok(ops) => input_operation = Some(Box::new(ops)),
                        Err(invalid_args) => {
                            eprintln!("error: {invalid_args}");
                            print_usage_and_exit();
                        }
                    };
                } else if args.is_empty() {
                    let ops_args = vec![other.to_string()];
                    match WriteFileOperation::init_from_arg(ops_args) {
                        Ok(ops) => output_operation = Some(Box::new(ops)),
                        Err(invalid_args) => {
                            eprintln!("error: {invalid_args}");
                            print_usage_and_exit();
                        }
                    };
                } else {
                    print_usage_and_exit();
                }
            }
        }
    }
    if input_operation.is_none() {
        eprintln!("error: No Input File");
        print_usage_and_exit();
    }
    if output_operation.is_none() {
        eprintln!("error: No Ouput File");
        print_usage_and_exit();
    }
    let source_image = input_operation.unwrap().apply();
    let mut final_image = source_image;

    for transformation in transforms.iter() {
        let img = transformation.apply(final_image);
        final_image = img;
    }
    output_operation.unwrap().apply(final_image);
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
