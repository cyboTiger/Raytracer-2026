use crate::{
    hittable::HitRecord,
    rtweekend::ray::{Point, Ray, random_unit_point, reflect, dot},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Point,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Point,
}

impl Lambertian {
    pub fn new(albedo: Point) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Point,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_point();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Point,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Point, fuzz: f64) -> Self {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }}
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Point,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(&r_in.dir, &rec.normal);
        reflected = reflected.unit_vector() + random_unit_point() * self.fuzz;
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        
        dot(scattered.dir, rec.normal) > 0.0
    }
}
