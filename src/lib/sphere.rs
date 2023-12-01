use glam::{DVec3,DVec2};
use crate::lib::triangle::Object;

use super::{Material, SpecularProperties};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub radius2: f64,
    pub material: Material,
    pub ior: f64,
    pub specular: SpecularProperties,
    pub diffuse_color: DVec3,
}
#[derive(Debug)]
pub enum SolveError {
    NoSolution,
}
#[allow(dead_code)]
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Result<(f64, f64), SolveError> {
    let discr = b * b - 4. * a * c;
    let x0;let x1;
    match discr {
        discr if discr == 0. => {
            x0 = -0.5 * b / a;
            x1 = x0.clone();
        },
        discr if discr < 0. => {
            return Err(SolveError::NoSolution)
        },
        _ => {
            let q = if b > 0. {-0.5 * (b + discr.sqrt())} else {-0.5 * (b - discr.sqrt())};
            x0 = q / a;
            x1 = c / q;
        }
    }
    match x0 < x1 {
        true => Ok((x0, x1)),
        false => Ok((x1, x0)),
    }
}

impl Object for Sphere {
    fn intersection(&self, light: super::Light, dir: DVec3) -> (bool, f64, usize, DVec2) {
        let l = light.org - self.center;
        let a = dir.dot(dir);
        let b = 2. * l.dot(dir);
        let c = l.dot(l) - self.radius2;
        match solve_quadratic(a, b, c) {
            Ok((t0, t1)) => {
                if t1 < 0. {
                    (false, t1, 0, DVec2::ZERO)
                }else if t0 < 0. {
                    (true, t1, 0, DVec2::ZERO)
                } else {
                    (true, t0, 0, DVec2::ZERO)
                }
            },
            Err(_) => (false, 0., 0, DVec2::ZERO),
        }
    }

    fn get_surface_properties(&self, p:DVec3, _px:DVec3, _idx:usize, _uv:DVec2) -> (DVec3, DVec2) {
        ((p - self.center).normalize(), DVec2::new(0., 0.))
    }
    fn get_material_properties(&self) -> super::Material {
        self.material
    }
    fn get_ior(&self) -> f64 {
        self.ior
    }
    fn get_specular_properties(&self) -> SpecularProperties {
        self.specular
    }
    fn eval_diffuse_color(&self, _vx: DVec2) -> DVec3 {
        self.diffuse_color
    }
}
#[cfg(test)]
mod tests {
    use glam::{DVec3, DVec2};

    use crate::lib::{Light, solve_quadratic, Object, Material, SpecularProperties};

    use super::Sphere;

    #[test]
    fn test_travel() {
        // let light = Light { org: DVec3::new(0., 0., 0.), dir: DVec3::new(1., 0., 0.)};
        // let sp = Sphere { center: DVec3::new(2., 0., 0.), radius: 1., radius2: 1.};
        match solve_quadratic(1., 2., 1.) {
            Ok(sp) => {
                // dbg!(sp);
                assert_eq!(sp.0, -1.);
                assert_eq!(sp.1, -1.);
            },
            Err(_) => {panic!("Error solving quadratic")},
        };
    }
    #[test]
    fn test_intersection() {
        let light = Light { org: DVec3::new(0., 0., 0.), inten: DVec3::new(0.88, 0.42, 0.)};
        let sp = Sphere { center: DVec3::new(2., 0., 0.), radius: 1.6, radius2: 2.56, material: Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25.0, 0.8, 0.2), diffuse_color: DVec3::splat(0.2)};
        let (res, t, b1, b2) = sp.intersection(light, DVec3::new(0.88, 0.42, 0.));
        assert_eq!(res, true);
        assert_eq!(t, 0.4683376845365324);
        assert_eq!(b1, 0);
        assert_eq!(b2, DVec2::ZERO);
    }
    #[test]
    fn test_eval_diffuse_color() {
        // let light = Light { org: DVec3::new(0., 0., 0.), dir: DVec3::new(0.88, 0.42, 0.)};
        let sp = Sphere { center: DVec3::new(2., 0., 0.), radius: 1.6, radius2: 2.56, material: Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25., 0.8, 0.2), diffuse_color: DVec3::splat(0.2) };
        let ss = sp.eval_diffuse_color(DVec2::new(1.2, 3.4));
        // dbg!(ss);
        assert_eq!(DVec3::new(0.815, 0.235, 0.031), ss);
    }
    #[test]
    fn test_get_surface_properties() {
        // let light = Light { org: DVec3::new(0., 0., 0.), dir: DVec3::new(0.88, 0.42, 0.)};
        let sp = Sphere { center: DVec3::new(2., 0., 0.), radius: 1.6, radius2: 2.56, material: Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25.0, 0.8, 0.2), diffuse_color: DVec3::splat(0.2) };
        let p = DVec3::new(0., 0., 0.);
        assert_eq!(sp.get_surface_properties(p, DVec3::new(0., 0., 0.), 0usize, DVec2::new(0., 0.)).0, DVec3::new(-1., 0., 0.))
    }
}