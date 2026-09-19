use crate::material;
use crate::rtweekend::interval;
use crate::rtweekend::ray;
use std::rc::Rc;
use std::vec;

pub struct HitRecord {
    pub p: ray::Point,
    pub normal: ray::Point,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Rc<dyn material::Material>>,
}

impl HitRecord {
    pub fn new(mat: Option<Rc<dyn material::Material>>) -> Self {
        HitRecord {
            p: ray::Point(0.0, 0.0, 0.0),
            normal: ray::Point(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
            mat,
        }
    }
    pub fn set(&mut self, p: ray::Point, normal: ray::Point, t: f64) {
        self.p = p;
        self.normal = normal;
        self.t = t;
    }

    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &ray::Point) {
        self.front_face = ray::dot(r.dir, *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, interval: &interval::Interval, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: ray::Point,
    radius: f64,
    mat: Rc<dyn material::Material>,
}

impl Sphere {
    pub fn new(center: ray::Point, radius: f64, mat: Rc<dyn material::Material>) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, interval: &interval::Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.orig;
        let a = r.dir.norm_squared();
        let h = ray::dot(r.dir, oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;
        rec.set(r.at(root), (r.at(root) - self.center) / self.radius, root);
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn new_with_item(object: Box<dyn Hittable>) -> Self {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, interval: &interval::Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;
        let tmp_rec = rec;

        for obj in &self.objects {
            if obj.hit(
                r,
                &interval::Interval::new(interval.min, closest_so_far),
                tmp_rec,
            ) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
            }
        }
        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
