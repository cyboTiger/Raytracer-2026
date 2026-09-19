use std::{f64::consts::PI, rc::Rc};

use crate::{material::Lambertian, rtweekend::ray::Point};
use console::style;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod rtweekend;

fn main() {
    let path = std::path::Path::new("output/book1/image19.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let r = (PI / 4.0).cos();
    let mut world = hittable::HittableList::new();

    let material_left = Lambertian::new(Point(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Point(1.0, 0.0, 0.0));

    world.add(Box::new(hittable::Sphere::new(
        Point(-r, 0.0, -1.0),
        r,
        Rc::new(material_left),
    )));
    world.add(Box::new(hittable::Sphere::new(
        Point(r, 0.0, -1.0),
        r,
        Rc::new(material_right),
    )));

    let mut cam = camera::Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 90.0;
    cam.render(&world);

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    cam.save_img(path);
}
