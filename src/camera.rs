use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use crate::hittable;
use crate::rtweekend;
use crate::rtweekend::interval;
use crate::rtweekend::random_double;
use crate::rtweekend::ray::Point;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::ray::random_on_hemisphere;
use crate::rtweekend::ray::random_unit_point;
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub img: Option<RgbImage>,
    pub max_depth: i32, // Maximum number of ray bounces into scene

    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Point,
    pixel_delta_v: Point,
    pixel_samples_scale: f64,
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
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world)
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
        self.center = Point(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Point(viewport_width, 0.0, 0.0);
        let viewport_v = Point(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Point(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // pixel sample scale
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn hittable::Hittable) -> Point {
        if depth <= 0 {
            return Point(0.0, 0.0, 0.0);
        }
        let mut tmp_rec = hittable::HitRecord::new();
        if world.hit(
            r,
            &rtweekend::interval::Interval {
                min: 0.001,
                max: rtweekend::INFINITY,
            },
            &mut tmp_rec,
        ) {
            // let direction = random_on_hemisphere(&tmp_rec.normal);
            let direction = tmp_rec.normal + random_unit_point();
            return self.ray_color(&Ray::new(tmp_rec.p, direction), depth - 1, world) * 0.5;
            // return (tmp_rec.normal + Point(1.0, 1.0, 1.0)) / 2.0;
        }
        let unit_dir = r.dir.unit_vector();
        let a = (unit_dir.1 + 1.0) * 0.5;
        rtweekend::ray::Point(1.0, 1.0, 1.0) * (1.0 - a) + rtweekend::ray::Point(0.5, 0.7, 1.0) * a
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

        let r: f64 = INTENSITY.clamp(color.0) * 255.999;
        let g: f64 = INTENSITY.clamp(color.1) * 255.999;
        let b: f64 = INTENSITY.clamp(color.2) * 255.999;
        *pixel = image::Rgb([r as u8, g as u8, b as u8]);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_center = self.pixel00_loc
            + self.pixel_delta_u * (i as f64 + offset.0)
            + self.pixel_delta_v * (j as f64 + offset.1);
        Ray::new(self.center, pixel_center - self.center)
    }

    fn sample_square(&self) -> Point {
        Point(random_double() - 0.5, random_double() - 0.5, 0.0)
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
