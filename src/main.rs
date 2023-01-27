use image::{io::Reader, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use std::env;

mod utils;
use utils::dimensions::calc_dimensions;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input = String::new();
    let mut strength = 0;

    for (i, item) in args.iter().enumerate() {
        if item == "-i" {
            input = args.get(i + 1).unwrap().to_owned();
        }
        if item == "-s" {
            strength = args.get(i + 1).unwrap().parse().unwrap();
        }
    }

    if input.len() <= 0 {
        panic!("provide the input");
    }

    let file_extension = input.split('.').last().unwrap();

    let image = Reader::open(&input).unwrap().decode().unwrap();

    let dimensions = calc_dimensions(&strength).unwrap();

    let (image_dimensions_x, image_dimensions_y) = image.dimensions();
    let mut blurred_image: RgbImage = ImageBuffer::new(image_dimensions_x, image_dimensions_y);

    for pixel in image.pixels() {
        let (sum_red, sum_green, sum_blue) =
            dimensions
                .iter()
                .fold((0u64, 0u64, 0u64), |prev, (plus_x, plus_y)| {
                    let mut add_pixel_x = pixel.0 as i32 + *plus_x;
                    let mut add_pixel_y = pixel.1 as i32 + *plus_y;

                    // normalize the target dimensions
                    if add_pixel_x < 0 {
                        add_pixel_x = 0;
                    }
                    if add_pixel_y < 0 {
                        add_pixel_y = 0;
                    }

                    if add_pixel_x >= image_dimensions_x.try_into().unwrap() {
                        add_pixel_x = image_dimensions_x as i32 - 1;
                    }
                    if add_pixel_y >= image_dimensions_y.try_into().unwrap() {
                        add_pixel_y = image_dimensions_y as i32 - 1
                    }

                    let target = image.get_pixel(
                        add_pixel_x.try_into().unwrap(),
                        add_pixel_y.try_into().unwrap(),
                    );

                    let (prev_red, prev_green, prev_blue) = prev;
                    let [new_red, new_green, new_blue, _] = target.channels() else { panic!("couldn't get channels") };

                    (
                        prev_red + *new_red as u64,
                        prev_green + *new_green as u64,
                        prev_blue + *new_blue as u64,
                    )
                });

        let red = sum_red / dimensions.len() as u64;
        let green = sum_green / dimensions.len() as u64;
        let blue = sum_blue / dimensions.len() as u64;

        let blurred_pixel = Rgb::<u8>([red as u8, green as u8, blue as u8]);

        blurred_image.put_pixel(pixel.0, pixel.1, blurred_pixel);
    }

    blurred_image.save(format!("out.{file_extension}")).unwrap();
}
