use image::{imageops::FilterType, DynamicImage, ImageBuffer};

use crate::{aspect_ratio::AspectRatio, Resolution};

pub fn new_dimensions(aspect_ratio: AspectRatio, resolution: &Resolution) -> Resolution {
    let desired_resolution = aspect_ratio.to_resolution();
    let desired_aspect_ratio = desired_resolution.aspect_ratio();
    let current_aspect_ratio = resolution.aspect_ratio();

    if (desired_aspect_ratio - current_aspect_ratio).abs() < 0.01 {
        Resolution {
            width: desired_resolution.width,
            height: desired_resolution.height,
        }
    } else if desired_aspect_ratio > current_aspect_ratio {
        // the image is too tall, keep the width, pad the width
        Resolution {
            width: (desired_resolution.height as f64 * current_aspect_ratio) as u32,
            height: desired_resolution.height,
        }
    } else {
        // the image is too wide, keep the height, pad the height
        Resolution {
            width: resolution.width,
            height: (desired_resolution.width as f64 / current_aspect_ratio) as u32,
        }
    }
}

pub fn resize_image(image: &DynamicImage, resolution: &Resolution) -> DynamicImage {
    image.resize(
        resolution.width as u32,
        resolution.height as u32,
        FilterType::Lanczos3,
    )
}

pub fn pad_image(
    image: DynamicImage,
    aspect_ratio: AspectRatio,
    pad_color: image::Rgb<u8>,
) -> DynamicImage {
    let resolution = aspect_ratio.to_resolution();
    let buffer = image.to_rgb8();
    let mut padded_image = ImageBuffer::from_pixel(resolution.width, resolution.height, pad_color);
    let x_offset = (resolution.width - image.width()) / 2;
    let y_offset = (resolution.height - image.height()) / 2;

    for (x, y, pixel) in padded_image.enumerate_pixels_mut() {
        if x >= x_offset
            && x < x_offset + image.width()
            && y >= y_offset
            && y < y_offset + image.height()
        {
            let image_pixel = buffer.get_pixel(x - x_offset, y - y_offset);
            *pixel = *image_pixel;
        } else {
            *pixel = pad_color;
        }
    }
    DynamicImage::ImageRgb8(padded_image)
}
