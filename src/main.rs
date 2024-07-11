use std::{
    collections::HashMap,
    io::{self, Write},
    path::Path,
    usize,
};

use itertools::Itertools;
use oxcii::{
    ascii_converter::ascii_converter::AsciiConversionMethod,
    image_processor::image_processor::ImageProcessor,
};
mod ascii_converter;
use strum::IntoEnumIterator;

fn main() {
    println!(
        "Welcome to Oxii. This program converts images to ascii and prints it to your terminal."
    );

    let (image_path, terminal_width, conversion_method) = read_inputs();
    let mut image_processor = ImageProcessor::new(image_path, terminal_width);
    image_processor.pre_process_image();
    image_processor.print_ascii(conversion_method);
}

fn read_inputs() -> (String, usize, AsciiConversionMethod) {
    let image_path = read_image_path();
    let terminal_width = read_terminal_width();
    let conversion_method = read_conversion_method();

    (image_path, terminal_width, conversion_method)
}

fn read_image_path() -> String {
    print!("Please enter an image path: ");
    io::stdout().flush().unwrap();
    let mut image_path = String::new();
    io::stdin()
        .read_line(&mut image_path)
        .expect("Must provide an image path");
    image_path = image_path.trim().to_string();

    let path_exists = Path::new(&image_path).exists();
    if !path_exists {
        panic!("Image at path {image_path} does not exist");
    }
    image_path
}

fn read_terminal_width() -> usize {
    print!("Please enter character width of terminal (default 100): ");
    io::stdout().flush().unwrap();
    let mut terminal_width_input = String::new();
    io::stdin().read_line(&mut terminal_width_input).unwrap();
    terminal_width_input = terminal_width_input.trim().to_string();

    let terminal_width: usize;
    if terminal_width_input.is_empty() {
        terminal_width = 100;
    } else {
        terminal_width = terminal_width_input
            .parse()
            .expect("Input is not a positive integer");
        assert!(terminal_width > 0, "Input must be greater than 0");
    }
    terminal_width
}

fn read_conversion_method() -> AsciiConversionMethod {
    let mut conversion_methods: HashMap<usize, AsciiConversionMethod> =
        AsciiConversionMethod::iter().enumerate().collect();
    println!("Select an Ascii conversion method:");
    conversion_methods
        .iter()
        .sorted_by_key(|x| x.0)
        .for_each(|(index, method)| {
            let mut index = index.clone();
            index += 1;
            println!("{index} - {method}");
        });

    print!("Please enter the index of the conversion method (e.g. 1 or 2): ");
    io::stdout().flush().unwrap();
    let mut conversion_method_index = String::new();
    io::stdin().read_line(&mut conversion_method_index).unwrap();
    conversion_method_index = conversion_method_index.trim().to_string();
    let mut conversion_method_index: usize = conversion_method_index
        .parse()
        .expect("Not a valid positive integer!");
    conversion_method_index -= 1;
    let conversion_method = conversion_methods
        .remove(&conversion_method_index)
        .expect("Not a valid index for conversion methods!");
    conversion_method
}
