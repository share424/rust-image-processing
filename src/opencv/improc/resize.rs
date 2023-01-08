use crate::image::{Image, Pixel};

fn nn_interpolate(image: &mut Image, x: f64, y: f64, channel: usize) -> u8 {
    return image.get_individual_pixel(x.round() as usize, y.round() as usize, channel);
}

pub fn nn_resize(image: &mut Image, w: usize, h: usize) -> Image {
    let mut resized_image = Image::new(w, h);
    let x_step = image.width as f64 / w as f64;
    let y_step = image.height as f64 / h as f64;

    let x_offset = (x_step / 2.0) - 0.5;
    let y_offset = (y_step / 2.0) - 0.5;

    for x in 0..w {
        for y in 0..h {
            let pixel = Pixel {
                r: nn_interpolate(image, x as f64 * x_step + x_offset, y as f64 * y_step + y_offset, 0),
                g: nn_interpolate(image, x as f64 * x_step + x_offset, y as f64 * y_step + y_offset, 1),
                b: nn_interpolate(image, x as f64 * x_step + x_offset, y as f64 * y_step + y_offset, 2),
            };
            resized_image.set_pixel(x, y, pixel);
        }
    }

    return resized_image;
}

fn bilinear_interpolate(image: &mut Image, x: f64, y: f64, channel: usize) -> u8 {
    let lower_x = x.floor();
    let upper_x = x.ceil();
    let lower_y = y.floor();
    let upper_y = y.ceil();

    let v1 = image.get_individual_pixel(lower_x as usize, lower_y as usize, channel) as f64 / 255.0;
    let v2 = image.get_individual_pixel(upper_x as usize, lower_y as usize, channel) as f64 / 255.0;
    let v3 = image.get_individual_pixel(lower_x as usize, upper_y as usize, channel) as f64 / 255.0;
    let v4 = image.get_individual_pixel(upper_x as usize, upper_y as usize, channel) as f64 / 255.0;

    let a1 = (upper_x - x) * (upper_y - y);
    let a2 = (x - lower_x) * (upper_y - y);
    let a3 = (upper_x - x) * (y - lower_y);
    let a4 = (x - lower_x) * (y - lower_y);

    return ((v1 * a1 + v2 * a2 + v3 * a3 + v4 * a4) * 255.0).floor() as u8;
}

pub fn bilinear_resize(image: &mut Image, w: usize, h: usize) -> Image {
    let mut resized_image = Image::new(w, h);
    let x_step = image.width as f64 / w as f64;
    let y_step = image.height as f64 / h as f64;

    let x_offset = (x_step / 2.0) - 0.5;
    let y_offset = (y_step / 2.0) - 0.5;

    for x in 0..w {
        for y in 0..h {
            let pixel = Pixel {
                r: bilinear_interpolate(image, x as f64 * x_step + x_offset, y as f64 * y_step + y_offset, 0),
                g: bilinear_interpolate(image, x as f64 * x_step + x_offset, y as f64 * y_step + y_offset, 1),
                b: bilinear_interpolate(image, x as f64 * x_step + x_offset, y as f64 * y_step + y_offset, 2),
            };
            resized_image.set_pixel(x, y, pixel);
        }
    }

    return resized_image;
}
