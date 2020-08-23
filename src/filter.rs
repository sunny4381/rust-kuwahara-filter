use image::{RgbaImage, Rgba};

use crate::error::Error;
use crate::iter::ImageIterator;

pub fn kuwahara(img: &mut RgbaImage, num: u32) -> Result<(), Error> {
    let (width, height) = img.dimensions();
    if width < num * 2 + 1 || height < num * 2 + 1 {
        // too small to process
        return Err(Error::FilterError(String::from("email_from")));
    }

    let calc_avg = |x: u32, y: u32| -> Rgba<u8> {
        let mut sum_r: u64 = 0;
        let mut sum_g: u64 = 0;
        let mut sum_b: u64 = 0;
        let mut sum_a: u64 = 0;

        for (i, j) in ImageIterator::new(num + 1, num + 1) {
            let px = img.get_pixel(x + i, y + j);
            sum_r += px[0] as u64;
            sum_g += px[1] as u64;
            sum_b += px[2] as u64;
            sum_a += px[3] as u64;
        }

        let avg_r: f64 = sum_r as f64 / (num + 1) as f64 / (num + 1) as f64;
        let avg_g: f64 = sum_g as f64 / (num + 1) as f64 / (num + 1) as f64;
        let avg_b: f64 = sum_b as f64 / (num + 1) as f64 / (num + 1) as f64;
        let avg_a: f64 = sum_a as f64 / (num + 1) as f64 / (num + 1) as f64;
        Rgba([avg_r as u8, avg_g as u8, avg_b as u8, avg_a as u8])
    };

    let calc_var = |x: u32, y: u32, avg: &Rgba<u8>| -> f64 {
        let mut sum_r: f64 = 0.0;
        let mut sum_g: f64 = 0.0;
        let mut sum_b: f64 = 0.0;
        let mut sum_a: f64 = 0.0;

        for i in 0..(num + 1) {
            for j in 0..(num + 1) {
                let px = img.get_pixel(x + i, y + j);
                sum_r += (px[0] as f64 - avg[0] as f64).powf(2.0);
                sum_g += (px[1] as f64 - avg[1] as f64).powf(2.0);
                sum_b += (px[2] as f64 - avg[2] as f64).powf(2.0);
                sum_a += (px[3] as f64 - avg[3] as f64).powf(2.0);
            }
        }

        let var_r = sum_r / (num + 1) as f64 / (num + 1) as f64;
        let var_g = sum_g / (num + 1) as f64 / (num + 1) as f64;
        let var_b = sum_b / (num + 1) as f64 / (num + 1) as f64;
        let var_a = sum_a / (num + 1) as f64 / (num + 1) as f64;

        var_r + var_g + var_b + var_a
    };

    let mut work_pixels: Vec<(u8, u8, u8, u8, f64)> = vec![(0, 0, 0, 0, 0.0); ((width - num) * (height - num)) as usize];
    let work_pixel_at = |x: u32, y: u32| -> usize {
        assert!(x < width - num, "width {} is out of range (max = {})", x, width);
        assert!(y < height - num, "height {} is out of range (max = {})", y, height);
        (y * (width - num) + x) as usize
    };

    for (x, y) in ImageIterator::new(width - num, height - num) {
        let avg = calc_avg(x, y);
        let var = calc_var(x, y, &avg);

        work_pixels[work_pixel_at(x, y)] = (avg[0], avg[1], avg[2], avg[3], var);
    }

    let min_tuple = |lhs: Option<(u8, u8, u8, u8, f64)>, rhs: Option<(u8, u8, u8, u8, f64)>| -> Option<(u8, u8, u8, u8, f64)> {
        match (lhs, rhs) {
            (Some(x), Some(y)) => if x.4 <= y.4 { Some(x) } else { Some(y) },
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            _ => None
        }
    };

    for (x, y) in ImageIterator::new(width, height) {
        let top_left = if x >= num && y >= num {
            Some(work_pixels[work_pixel_at(x - num, y - num)])
        } else {
            None
        };
        let top_right = if x < width - num && y >= num {
            Some(work_pixels[work_pixel_at(x, y - num)])
        } else {
            None
        };
        let bottom_left = if x >= num && y < height - num {
            Some(work_pixels[work_pixel_at(x - num, y)])
        } else {
            None
        };
        let bottom_right = if x < width - num && y < height - num {
            Some(work_pixels[work_pixel_at(x, y)])
        } else {
            None
        };

        let min = min_tuple(min_tuple(top_left, top_right), min_tuple(bottom_left, bottom_right));
        if let Some(pixel) = min {
            img.put_pixel(x, y, Rgba([pixel.0, pixel.1, pixel.2, pixel.3]))
        }
    }

    Ok(())
}
