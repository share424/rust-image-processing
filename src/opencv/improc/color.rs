use crate::image::{Image, Pixel, ColorMode};
use std::{cmp::{min, max}, f64::consts::PI};

pub fn rgb_to_grayscale(image: &mut Image) { 
    for x in 0..image.width {
        for y in 0..image.height {
            let pixel = image.get_pixel(x.into(), y.into());
            let value = (0.299 * pixel.r as f64) + (0.587 * pixel.g as f64) + (0.114 * pixel.b as f64);
            image.set_pixel(x.into(), y.into(), Pixel {
                r: value.floor() as u8,
                g: value.floor() as u8,
                b: value.floor() as u8
            });
            image.color_mode = ColorMode::GRAY;
        }
    }
}

pub fn shift_image(image: &mut Image, value: i16, clamp: bool) {
    shift_image_channel(image, 0, value, clamp);
    shift_image_channel(image, 1, value, clamp);
    shift_image_channel(image, 2, value, clamp);
}

pub fn shift_image_channel(image: &mut Image, channel: usize, value: i16, clamp: bool) {
    for x in 0..image.width {
        for y in 0..image.height {
            let mut v = image.get_individual_pixel(x.into(), y.into(), channel);
            v = if clamp { 
                min(max(v as i16 + value as i16, 0), 255) as u8
            } else { 
                (v as i16).overflowing_add(value).0 as u8
            };
            
            image.set_individual_pixel(x.into(), y.into(), channel, v);
        }
    }
}

pub fn scale_image_channel(image: &mut Image, channel: usize, value: f64, clamp: bool) {
    for x in 0..image.width {
        for y in 0..image.height {
            let mut v = image.get_individual_pixel(x.into(), y.into(), channel);
            v = if clamp { 
                min_float(max_float(v as f64 * value, 0.0), 255.0) as u8
            } else { 
                max_float(v as f64 * value , 255.0) as u8
            };
            
            image.set_individual_pixel(x.into(), y.into(), channel, v);
        }
    }
}

fn max_float(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn min_float(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn three_way_max(a: f64, b: f64, c: f64) -> f64 {
    if a > b {
        if a > c {
            return a;
        } else {
            return c;
        }
    } else {
        if b > c {
            return b;
        } else {
            return c;
        }
    }
}

fn three_way_min(a: f64, b: f64, c: f64) -> f64 {
    if a < b {
        if a < c {
            return a;
        } else {
            return c;
        }
    } else {
        if b < c {
            return b;
        } else {
            return c;
        }
    }
}

pub fn rgb_to_hsv(image: &mut Image) {
    for x in 0..image.width {
        for y in 0..image.height {
            let pixel = image.get_pixel(x.into(), y.into());
            let r = pixel.r as f64 / 255.0 as f64;
            let g = pixel.g as f64 / 255.0 as f64;
            let b = pixel.b as f64 / 255.0 as f64;

            let value = three_way_max(r, g, b);
            let m = three_way_min(r, g, b);
            let c: f64 = value - m;
            
            let saturation = if value == 0f64 { 0f64 } else { c / value };

            let ha = if c == 0f64 {
                0f64
            } else if value == r {
                (g - b) / c
            } else if value == g {
                ((b - r) / c) + 2f64
            } else {
                ((r - g) / c) + 4f64
            };

            let mut hue = ha / 6f64;
            if ha < 0f64 {
                hue += 1f64;
            }

            image.set_pixel(x.into(), y.into(), Pixel {
                r: (hue * 255f64).floor() as u8,
                g: (saturation * 255f64).floor() as u8,
                b: (value * 255f64).floor() as u8
            });
            image.color_mode = ColorMode::HSV;
        }
    }
}

pub fn hsv_to_rgb(image: &mut Image) {
    for x in 0..image.width {
        for y in 0..image.height {
            let pixel = image.get_pixel(x.into(), y.into());
            let hue = pixel.r as f64 / 255f64;
            let saturation = pixel.g as f64 / 255f64;
            let value = pixel.b as f64 / 255f64;

            let r: f64;
            let g: f64;
            let b: f64;

            if saturation == 0f64 {
                r = value;
                g = value;
                b = value;
            } else {
                let mut h = hue * 6f64;
                if h == 0f64 {
                    h = 0f64;
                }
                let i = h.floor();

                let d = value * (1f64 - saturation);
                let e = value * (1f64 - saturation * (h - i));
                let f = value * (1f64 - saturation * (1f64 - (h - i)));

                match i as i32 {
                    0 => {
                        r = value; g = f; b = d;
                    }
                    1 => {
                        r = e; g = value; b = d;
                    }
                    2 => {
                        r = d; g = value; b = f;
                    }
                    3 => {
                        r = d; g = f; b = value;
                    }
                    4 => {
                        r = f; g = d; b = value;
                    }
                    _ => {
                        r = value; g = d; b = e;
                    }
                }
            }

            image.set_pixel(x.into(), y.into(), Pixel {
                r: (r * 255.0).floor() as u8,
                g: (g * 255.0).floor() as u8,
                b: (b * 255.0).floor() as u8
            });
            image.color_mode = ColorMode::RGB;
        }
    }
}

// TODO: maybe wrong implementation (?)
pub fn rgb_to_hcl(image: &mut Image) {
    for i in 0..image.width {
        for j in 0..image.height {
            let pixel = image.get_pixel(i.into(), j.into());
            let mut r = pixel.r as f64 / 255.0 as f64;
            let mut g = pixel.g as f64 / 255.0 as f64;
            let mut b = pixel.b as f64 / 255.0 as f64;

            // convert RGB to CIE-XYZ
            let gamma = 2.2;
            r = r.powf(gamma);
            g = g.powf(gamma);
            b = b.powf(gamma);

            // sRGB (D65) transformation matrix
            let mut x = r * 0.4124 + g * 0.3576 + b * 0.1805;
            let mut y = r * 0.2126 + g * 0.7152 + b * 0.0722;
            let mut z = r * 0.0193 + g * 0.1192 + b * 0.9505;

            // convert CIE-XYZ to CIE-Lab
            // XYZ reference: Observer: 2 degree, illuminant: D65 (Daylight, sRGB, Adobe-RGB)
            x = x / 95.047;
            y = y / 100.0;
            z = z / 108.883;

            let fx = if x > 0.008856 { x.powf(0.33333) } else { (903.3 * x + 16.0) / 116.0 };
            let fy = if y > 0.008856 { y.powf(0.33333) } else { (903.3 * y + 16.0) / 116.0 };
            let fz = if z > 0.008856 { z.powf(0.33333) } else { (903.3 * z + 16.0) / 116.0 };

            let mut l = 116.0 * fy - 16.0;
            let ca = 500.0 * (fx - fy);
            let cb = 200.0 * (fy - fz);

            // convert CIE-Lab to CIE-Lch
            let mut c = (ca * ca + cb * cb).sqrt();
            let mut h = cb.atan2(ca);
            h = if h > 0.0 { h / PI * 180.0 } else { 360.0 - h.abs() / PI * 180.0 };

            // normalize
            h /= 360.0;
            c /= 100.0;
            l /= 100.0;

            image.set_pixel(i.into(), j.into(), Pixel {
                r: (h * 255f64).floor() as u8,
                g: (c * 255f64).floor() as u8,
                b: (l * 255f64).floor() as u8
            });
            image.color_mode = ColorMode::HCL;
       }
    }
}

fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.0;
}

// TODO: maybe wrong implementation (?)
pub fn hcl_to_rgb(image: &mut Image) {
    for i in 0..image.width {
        for j in 0..image.height {
            let pixel = image.get_pixel(i.into(), j.into());
            let h = pixel.r as f64 / 255.0 as f64 * 360.0;
            let c = pixel.g as f64 / 255.0 as f64 * 100.0;
            let l = pixel.b as f64 / 255.0 as f64 * 100.0;

            // convert CIE-Lch to CIE-Lab
            let ca = deg_to_rad(h).cos() * c;
            let cb = deg_to_rad(h).sin() * c;

            // convert CIE-Lab to CIE-XYZ
            let mut y = (l + 16.0) / 116.0;
            let mut x = (ca / 500.0) + y;
            let mut z = y - (cb / 200.0);

            y = if y > 0.008856 { y.powf(3.0) } else { (y * 116.0 - 16.0) / 903.3 };
            x = if x > 0.008856 { x.powf(3.0) } else { (x * 116.0 - 16.0) / 903.3 };
            z = if z > 0.008856 { z.powf(3.0) } else { (z * 116.0 - 16.0) / 903.3 };

            // XYZ reference: Observer: 2 degree, illuminant: D65 (Daylight, sRGB, Adobe-RGB)
            x = x * 95.047;
            y = y * 100.0;
            z = z * 108.883;

            // convert CIE-XYZ to RGB
            // sRGB (D65) transformation inverse matrix
            let mut r = x * 3.2440 + y * -1.5371 + z * -0.4985;
            let mut g = x * -0.9692 + y * 1.8760 + z * 0.0415;
            let mut b = x * 0.0556 + y * -0.2040 + z * 1.0572;

            let gamma_b = 1.0 / 2.2;
            r = r.powf(gamma_b);
            g = g.powf(gamma_b);
            b = b.powf(gamma_b);

            image.set_pixel(i.into(), j.into(), Pixel {
                r: (r * 255.0).floor() as u8,
                g: (g * 255.0).floor() as u8,
                b: (b * 255.0).floor() as u8
            });
            image.color_mode = ColorMode::RGB;
       }
    }
}