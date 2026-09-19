use std::rc::Rc;

use crate::{
    hittable::Sphere,
    material::{Dielectric, Lambertian, Metal},
    rtweekend::{
        random_double, random_double_minmax,
        ray::{Point, random_point, random_point_minmax},
    },
};
use console::style;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod rtweekend;

fn main() {
    let path = std::path::Path::new("output/book1/image23.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let mut world = hittable::HittableList::new();

    let material_ground = Lambertian::new(Point(0.5, 0.5, 0.5));

    world.add(Box::new(hittable::Sphere::new(
        Point(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(material_ground),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material: Option<Rc<dyn material::Material>>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_point() * random_point();
                    sphere_material = Some(Rc::new(Lambertian::new(albedo)));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.unwrap())));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_point_minmax(0.5, 1.0);
                    let fuzz = random_double_minmax(0.0, 0.5);
                    sphere_material = Some(Rc::new(Metal::new(albedo, fuzz)));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.unwrap())));
                } else {
                    // glass
                    sphere_material = Some(Rc::new(Dielectric::new(1.5)));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.unwrap())));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Point(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Rc::new(Lambertian::new(Point(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Point(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Rc::new(Metal::new(Point(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Point(4.0, 1.0, 0.0), 1.0, mat3)));

    let mut cam = camera::Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point(13.0, 2.0, 3.0);
    cam.lookat = Point(0.0, 0.0, 0.0);
    cam.vup = Point(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&world);

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    cam.save_img(path);
}
