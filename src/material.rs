use crate::{
    hittable::HitRecord,
    rtweekend::ray::{Point, Ray, random_unit_point, reflect},
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
}

impl Metal {
    pub fn new(albedo: Point) -> Self {
        Metal { albedo }
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
        let reflected = reflect(&r_in.dir, &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}
