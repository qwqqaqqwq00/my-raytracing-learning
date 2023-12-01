use std::f64::INFINITY;
use core::marker::Copy;
use glam::{DVec3, DVec2};
use super::{Light, ObjectClone};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangle {
    pub v0: DVec3,
    pub v1: DVec3,
    pub v2: DVec3,
    pub s0: DVec2,
    pub s1: DVec2,
    pub s2: DVec2,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Material {
    DiffuseAndGlossy,
    ReflectionAndRefraction,
    Reflection,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpecularProperties(pub f64,pub f64,pub f64);
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MeshTriangle {
    pub vertices: Vec<Triangle>,
    pub material: Material,
    pub ior: f64,
    pub specular: SpecularProperties,
}
pub trait Object: ObjectClone {
    fn intersection(&self, light: Light, dir: DVec3) -> (bool, f64, usize, DVec2);
    fn eval_diffuse_color(&self, vx: DVec2) -> DVec3  {
        let scale = 5f64;
        let pattern = ((vx.x * scale) % 1f64 > 0.5) ^ ((vx.y * scale) % 1f64 > 0.5);
        DVec3::new(0.815, 0.235, 0.031).lerp(DVec3::new(0.937, 0.937, 0.231), f64::from(u32::from(pattern)))
    }
    fn get_surface_properties(&self, p:DVec3, px:DVec3, idx:usize, uv:DVec2) -> (DVec3, DVec2);
    fn get_material_properties(&self) -> Material;
    fn get_ior(&self) -> f64;
    fn get_specular_properties(&self) -> SpecularProperties;
}
#[allow(dead_code)]
impl Object for MeshTriangle {
    fn intersection(&self, light: Light, dir: DVec3) -> (bool, f64, usize, DVec2) {
        let mut isec = false;let mut t = INFINITY;let mut b1 = 0.;let mut b2 = 0.;let mut ix: usize = 0;
        self.vertices.iter().enumerate().for_each(|(i, x)| {
            let (cond, tn, b1t, b2t) = light.intersection(x.v0, x.v1, x.v2, dir);
            if cond && t > tn {
                isec |= cond;
                t = tn;
                ix = i;
                b1 = b1t;
                b2 = b2t;
            }
        });
        (isec, t, ix, DVec2::new(b1, b2))
    }
    fn get_ior(&self) -> f64 {
        self.ior
    }
    fn get_surface_properties(&self, _p:DVec3, _px:DVec3, idx: usize, uv: DVec2) -> (DVec3, DVec2) {
        let tri = self.vertices[idx];
        let e0 = (tri.v1 - tri.v0).normalize();
        let e1 = (tri.v2 - tri.v1).normalize();
        let n = e0.cross(e1).normalize();
        let st = (1. - uv.x - uv.y) * tri.s0 + uv.x * tri.s1 + uv.y * tri.s2;
        (n, st)
    }
    fn get_material_properties(&self) -> Material {
        self.material
    }
    fn get_specular_properties(&self) -> SpecularProperties {
        self.specular
    }
}

// impl Copy for MeshTriangle {
    // fn copy(&self) -> &Self {
        // todo!()
    // }
// }