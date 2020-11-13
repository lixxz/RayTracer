use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {x: self.x / other, y: self.y / other, z: self.z / other}
    }
}

impl Vec3 {
    fn normalize(&self) -> Vec3 {
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 {x: self.x / mag, y: self.y / mag, z: self.z / mag}
    }

    fn dot(&self, b: &Vec3) -> f64 {
        return self.x * b.x + self.y * b.y + self.z * b.z;
    }
}

#[derive(Copy, Clone)]
struct Ray {
    origin: Vec3,
    direction: Vec3
}

#[derive(Copy, Clone)]
struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    fn intersect(self, ray: Ray) -> (bool, f64) {
        let o = ray.origin;
        let d = ray.direction;
        let oc = o - self.center;
        let b = oc.dot(&d) * 2.0;
        let c = oc.dot(&oc) - self.radius * self.radius;
        let mut disc = b * b - c * 4.0;
        if disc < 0.0 { return (false, -1.0) };
        disc =  disc.sqrt();
        let t1 = -b  - disc;
        let t2 = -b + disc;
        let t = if t1 < t2 { t1 } else { t2 };
        return (true, t);
    }

    fn normal(self, p: Vec3) -> Vec3 {
        return (p - self.center) / self.radius;
    }
}

fn clamp(col: &mut Vec3) {
    col.x = if col.x > 255.0 { 255.0 } else if col.x < 0.0 { 0.0 } else { col.x };
    col.y = if col.y > 255.0 { 255.0 } else if col.y < 0.0 { 0.0 } else { col.y };
    col.z = if col.z > 255.0 { 255.0 } else if col.z < 0.0 { 0.0 } else { col.z };
}

fn main() -> std::io::Result<()> {
    let path = Path::new("image.ppm");
    let mut file = File::create(&path)?;

    let width = 600;
    let height = 600;

    let white = Vec3 {x: 255.0, y: 255.0, z: 255.0};
    let red = Vec3 {x: 255.0, y: 0.0, z: 0.0};

    let sphere = Sphere {center: Vec3 {x: width as f64 * 0.5, y: height as f64 * 0.5, z: 50.0}, radius: 50.0};
    let light = Sphere {center: Vec3 {x: 0.0, y: 0.0, z: 50.0}, radius: 1.0};

    let mut pixel_color = Vec3 {x: 0.0, y: 0.0, z: 0.0};

    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;

    for i in 0..height {
        for j in 0..width {
            let ray = Ray {origin: Vec3 {x: j as f64, y: i as f64, z: 0 as f64}, direction: Vec3 {x: 0.0, y: 0.0, z: 1.0} };
            
            let (does_intersect, t) = sphere.intersect(ray); 

            if does_intersect {
                let p = ray.origin + ray.direction * t;
                let l = light.center - p;
                let n = sphere.normal(p);

                let dt = l.normalize().dot(&n.normalize());

                pixel_color = (red + white * dt) * 0.5;
                clamp(&mut pixel_color);
            }

            file.write_all(format!("{} {} {}\n", pixel_color.x as u8, pixel_color.y as u8, pixel_color.z as u8).as_bytes())?;
        }
    }

    Ok(())
}