use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use crate::rtweekend::ray::Point;

pub mod rtweekend;
pub mod hittable;

// fn hit_sphere(center: &rtweekend::ray::Point, radius: f64, r: &rtweekend::ray::Ray) -> f64 {
//     let a = r.dir.norm_squared();
//     let h = r.dir * (*center - r.orig);
//     let c = (*center - r.orig).norm_squared() - radius * radius;
//     let discriminant = h * h - a * c;
//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-h - discriminant.sqrt()) / a
//     }
// }

fn ray_color(r: &rtweekend::ray::Ray, world: &dyn hittable::Hittable) -> rtweekend::ray::Point {
    let mut tmp_rec = hittable::HitRecord::new();
    if world.hit(r, 0.0, rtweekend::INFINITY, &mut tmp_rec) {
        return (tmp_rec.normal + Point(1.0, 1.0, 1.0)) / 2.0
    }
    let unit_dir = r.dir.unit_vector();
    let a = (unit_dir.1 + 1.0) * 0.5;
    rtweekend::ray::Point(1.0, 1.0, 1.0) * (1.0 - a) + rtweekend::ray::Point(0.5, 0.7, 1.0) * a
}

fn main() {
    let path = std::path::Path::new("output/book1/image5.png");
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
    let camera_center = rtweekend::ray::Point(0.0, 0.0, 0.0);

    // viewport
    let viewport_u = Point(viewport_width, 0.0, 0.0);
    let viewport_v = Point(0.0, -viewport_height, 0.0);

    // viewport delta
    let pixel_delta_u = viewport_u / width as f64;
    let pixel_delta_v = viewport_v / height as f64;

    // upper left pixel location
    let viewport_upper_left =
        camera_center - rtweekend::ray::Point(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let mut world = hittable::HittableList::new();
    world.add(Box::new(hittable::Sphere::new(Point(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(hittable::Sphere::new(Point(0.0, -100.5, -1.0), 100.0)));
    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;

            let ray = rtweekend::ray::Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray, &world);
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
