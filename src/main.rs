use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod vec;

fn ray_color(r: &vec::Ray) -> vec::Point {
    let unit_dir = r.dir / r.dir.l2norm();
    let a = (unit_dir.1 + 1.0) * 0.5;
    vec::Point(1.0, 1.0, 1.0) * (1.0 - a) + vec::Point(0.5, 0.7, 1.0) * a
}

fn main() {
    let path = std::path::Path::new("output/book1/image2.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = std::cmp::max((width as f64 / aspect_ratio) as u32, 1);

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let camera_center = vec::Point(0.0, 0.0, 0.0);

    // viewport
    let viewport_u = vec::Point(viewport_width, 0.0, 0.0);
    let viewport_v = vec::Point(0.0, -viewport_height, 0.0);

    // viewport delta
    let pixel_delta_u = viewport_u / width as f64;
    let pixel_delta_v = viewport_v / height as f64;

    // upper left pixel location
    let viewport_upper_left =
        camera_center - vec::Point(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;

            let ray = vec::Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray);
            let r: f64 = pixel_color.0 * 255.999;
            let g: f64 = pixel_color.1 * 255.999;
            let b: f64 = pixel_color.2 * 255.999;
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    img.save(path).expect("Cannot save the image to the file");
}
