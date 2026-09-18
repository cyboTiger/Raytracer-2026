use crate::rtweekend::ray::Point;
use console::style;

pub mod camera;
pub mod hittable;
pub mod rtweekend;

fn main() {
    let path = std::path::Path::new("output/book1/image11.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let mut world = hittable::HittableList::new();
    world.add(Box::new(hittable::Sphere::new(Point(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(hittable::Sphere::new(
        Point(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut cam = camera::Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    cam.save_img(path);
}
