use glam::{DVec3, DVec2};
use lib::{Scene, Sphere, ObjectAppend, SpecularProperties, MeshTriangle, Triangle, LightAppend, render};

mod lib;

fn main() {
    let mut sc = Scene::window(1280, 960);
    let sph1 = Sphere { center: DVec3::new(-1., 0., -12.), radius: 2., radius2: 4., material: lib::Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25.0, 0.8, 0.2), diffuse_color: DVec3::new(0.6, 0.7, 0.8)};
    let sph2 = Sphere { center: DVec3::new(0.5, -0.5, -8.), radius: 1.5, radius2: 2.25, material: lib::Material::ReflectionAndRefraction, ior: 1.5, specular: SpecularProperties(25.0, 0.8, 0.2), diffuse_color: DVec3::splat(0.2)};
    
    ObjectAppend::append(&mut sc, Box::new(sph1));
    ObjectAppend::append(&mut sc, Box::new(sph2));
    let verts = vec![
        DVec3::new(-5., -3., -6.),
        DVec3::new(5., -3., -6.),
        DVec3::new(5., -3., -16.),
        DVec3::new(-5., -3., -16.)
    ];
    let st = vec![
        DVec2::new(0., 0.),
        DVec2::new(1., 0.),
        DVec2::new(1., 1.),
        DVec2::new(0., 1.),
    ];
    let vertsx = vec![
        Triangle { v0: verts[0], v1: verts[1], v2: verts[3], s0: st[0], s1: st[1], s2: st[3] },
        Triangle { v0: verts[1], v1: verts[2], v2: verts[3], s0: st[1], s1: st[2], s2: st[3] },
    ];
    let mesh = MeshTriangle {
        vertices: vertsx,
        material: lib::Material::DiffuseAndGlossy,
        ior: 1.3,
        specular: SpecularProperties(25.0, 0.8, 0.2),
    };
    ObjectAppend::append(&mut sc, Box::new(mesh));
    LightAppend::append(&mut sc, lib::Light { org: DVec3::new(-20., 70., 20.), inten: DVec3::splat(0.5) });
    LightAppend::append(&mut sc, lib::Light { org: DVec3::new(30., 50., -12.), inten: DVec3::splat(0.5) });
    render(&mut sc);
}
#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn test_main() {
        main();
    }
}