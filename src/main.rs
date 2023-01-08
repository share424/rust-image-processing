use opencv::image::Image;
use opencv::improc::{color, resize};

fn main() {
    let mut image = Image::load("dog.jpg");

    println!("dimension: {}x{}", image.height, image.width);
    
    // Part 1. Color
    // color::shift_image(&mut image, 102, true);
    // color::shift_image_channel(&mut image, 0, 102, true);

    // color::rgb_to_hsv(&mut image);
    // color::scale_image_channel(&mut image, 1, 2.0, true);
    // color::hsv_to_rgb(&mut image);

    // color::rgb_to_grayscale(&mut image);

    // color::rgb_to_hcl(&mut image);
    // color::shift_image_channel(&mut image, 2, 200, true);
    // color::shift_image_channel(&mut image, 1, -3, true);
    // color::hcl_to_rgb(&mut image);

    // Part 2. Resize
    // let image = resize::nn_resize(&mut image, 1000, 1000);
    // let image = resize::bilinear_resize(&mut image, 1000, 1000);

    image.save("dog-output.jpg", 100).expect("Failed to save");
}
