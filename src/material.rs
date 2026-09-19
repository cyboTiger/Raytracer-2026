use crate::{
    hittable::HitRecord,
    rtweekend::ray::{Point, Ray, dot, random_unit_point, reflect, refract},
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Point, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Point,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Point(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = r_in.dir.unit_vector();
        let cos_theta = if dot(-unit_dir, rec.normal) < 1.0 {
            dot(-unit_dir, rec.normal)
        } else {
            1.0
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_reflect = ri * sin_theta > 1.0;
        let direction = if cannot_reflect {
            reflect(&unit_dir, &rec.normal)
        } else {
            refract(&unit_dir, &rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}
