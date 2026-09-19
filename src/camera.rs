use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use crate::hittable;
use crate::rtweekend;
use crate::rtweekend::degrees_to_radians;
use crate::rtweekend::interval;
use crate::rtweekend::linear_to_gamma;
use crate::rtweekend::random_double;
use crate::rtweekend::ray;
use crate::rtweekend::ray::Point;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::ray::cross;
use crate::rtweekend::ray::random_in_unit_disk;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub img: Option<RgbImage>,
    pub max_depth: i32, // Maximum number of ray bounces into scene
    pub vfov: f64,      // Vertical view angle (field of view)

    pub lookfrom: Point, // Point camera is looking from
    pub lookat: Point,   // Point camera is looking at
    pub vup: Point,      // Camera-relative "up" direction

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Point,
    pixel_delta_v: Point,
    pixel_samples_scale: f64,

    u: Point,
    v: Point,
    w: Point,

    defocus_disk_u: Point,
    defocus_disk_v: Point,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 100,
            center: Point(0.0, 0.0, 0.0),
            pixel00_loc: Point(0.0, 0.0, 0.0),
            pixel_delta_u: Point(0.0, 0.0, 0.0),
            pixel_delta_v: Point(0.0, 0.0, 0.0),
            samples_per_pixel: 10,
            pixel_samples_scale: 0.1,
            max_depth: 10,
            img: None,
            vfov: 90.0,
            lookfrom: Point(0.0, 0.0, 0.0),
            lookat: Point(0.0, 0.0, -1.0),
            vup: Point(0.0, 1.0, 0.0),
            u: Point::new(),
            v: Point::new(),
            w: Point::new(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Point::new(),
            defocus_disk_v: Point::new(),
        }
    }

    pub fn render(&mut self, world: &dyn hittable::Hittable) {
        self.initialize();
        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            // eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            // std::io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Point(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
                }

                self.write_color(i, j, pixel_color * self.pixel_samples_scale);
            }
            progress.inc(1);
        }
        progress.finish();

        eprintln!("\rDone.                 \n");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.img = Some(ImageBuffer::new(self.image_width, self.image_height));
        self.center = self.lookfrom;

        // Determine viewport dimensions.
        // let focal_length = (self.lookfrom - self.lookat).norm();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = (cross(self.vup, self.w)).unit_vector();
        self.v = cross(self.w, self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        // pixel sample scale
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn hittable::Hittable) -> Point {
        if depth <= 0 {
            return Point(0.0, 0.0, 0.0);
        }
        let mut tmp_rec = hittable::HitRecord::new(None);
        if world.hit(
            r,
            &rtweekend::interval::Interval {
                min: 0.001,
                max: rtweekend::INFINITY,
            },
            &mut tmp_rec,
        ) {
            let mut scattered = ray::Ray::default();
            let mut attenuation = Point::new();

            if tmp_rec
                .mat
                .as_ref()
                .expect("Record should have corresponding material")
                .scatter(r, &tmp_rec, &mut attenuation, &mut scattered)
            {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            return Point::new();
        }
        let unit_dir = r.dir.unit_vector();
        let a = (unit_dir.1 + 1.0) * 0.5;
        Point(1.0, 1.0, 1.0) * (1.0 - a) + Point(0.5, 0.7, 1.0) * a
    }

    fn write_color(&mut self, i: u32, j: u32, color: Point) {
        // let pixel = &self.img.unwrap().get_pixel_mut(i, j);
        let img = self
            .img
            .as_mut()
            .expect("initialize() must be called first");
        let pixel = img.get_pixel_mut(i, j);

        const INTENSITY: interval::Interval = interval::Interval {
            min: 0.0,
            max: 0.999,
        };

        let r: f64 = INTENSITY.clamp(linear_to_gamma(color.0)) * 255.999;
        let g: f64 = INTENSITY.clamp(linear_to_gamma(color.1)) * 255.999;
        let b: f64 = INTENSITY.clamp(linear_to_gamma(color.2)) * 255.999;
        *pixel = image::Rgb([r as u8, g as u8, b as u8]);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_center = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.0)
            + self.pixel_delta_v * (j as f64 + offset.1);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        Ray::new(ray_origin, pixel_center - ray_origin)
    }

    fn sample_square(&self) -> Point {
        Point(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        self.center + self.defocus_disk_u * p.0 + self.defocus_disk_v * p.1
    }

    pub fn save_img(&self, path: &std::path::Path) {
        let img = self
            .img
            .as_ref()
            .expect("initialize() must be called first");
        let _ = img.save(path);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
