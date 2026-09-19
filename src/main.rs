use std::rc::Rc;

use crate::{
    material::{Dielectric, Lambertian, Metal},
    rtweekend::ray::Point,
};
use console::style;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod rtweekend;

fn main() {
    let path = std::path::Path::new("output/book1/image18.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let mut world = hittable::HittableList::new();

    let material_ground = Lambertian::new(Point(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Point(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Point(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(hittable::Sphere::new(
        Point(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    )));
    world.add(Box::new(hittable::Sphere::new(
        Point(0.0, 0.0, -1.2),
        0.5,
        Rc::new(material_center),
    )));
    world.add(Box::new(hittable::Sphere::new(
        Point(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    )));
    world.add(Box::new(hittable::Sphere::new(
        Point(-1.0, 0.0, -1.0),
        0.4,
        Rc::new(material_bubble),
    )));
    world.add(Box::new(hittable::Sphere::new(
        Point(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
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
