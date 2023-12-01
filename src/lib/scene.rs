use glam::DVec3;
use super::{Object, Light};

pub trait ObjectClone {
    fn clone_box(&self) -> Box<dyn Object>;
}

impl<T> ObjectClone for T
where
    T: 'static + Object + Clone,
{
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        self.clone_box()
    }
}
#[allow(dead_code)]
pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub fov: f64,
    pub background_color: DVec3,
    pub max_depth: i16,
    pub epsilon: f64,
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Light>,
}
pub trait ObjectAppend {
    fn append(&mut self, obj: Box<dyn Object>);
}
pub trait LightAppend {
    fn append(&mut self, light: Light);
}
#[allow(dead_code)]
impl Scene {
    pub fn new(width: i32, height: i32, fov: f64, background_color: DVec3, max_depth: i16, epsilon: f64, objects: Vec<Box<dyn Object>>, lights: Vec<Light>) -> Self {
        Self { width, height, fov, background_color, max_depth, epsilon, objects, lights }
    }
    pub fn create() -> Self {
        Scene::new(1280, 960, 90., DVec3::new(0.235294, 0.67451, 0.843137), 5, 0.00001, Vec::<Box<dyn Object>>::new(), Vec::<Light>::new())
    }
    pub fn window(width: i32, height: i32) -> Self {
        Scene::new(width, height, 90., DVec3::new(0.235294, 0.67451, 0.843137), 5, 0.00001, Vec::<Box<dyn Object>>::new(), Vec::<Light>::new())
    }
    pub fn get_obj(&self) -> &Vec<Box<dyn Object>> {
        &self.objects
    }
    pub fn get_light(&self) -> &Vec<Light> {
        &self.lights
    }
}
impl ObjectAppend for Scene {
    fn append(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }
}
impl LightAppend for Scene {
    fn append(&mut self, light: Light) {
        self.lights.push(light);
    }
}

#[cfg(test)]
mod tests {
    use glam::{DVec3, DVec2};

    use crate::lib::{Sphere, MeshTriangle, Triangle, Light, SpecularProperties};

    use super::{Scene, ObjectAppend, LightAppend};

    #[test]
    fn test_append_obj() {
        let mut sc = Scene::create();
        let tri = MeshTriangle { vertices: vec![Triangle { 
            v0: DVec3::ZERO, v1: DVec3::ZERO, v2: DVec3::ZERO, s0: DVec2::ZERO, s1: DVec2::ZERO, s2: DVec2::ZERO 
        }],
            material: crate::lib::Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25.0, 0.8, 0.2) 
        };
        let sp = Sphere { center: DVec3::new(2., 0., 0.), radius: 1.6, radius2: 2.56, material: crate::lib::Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25.0, 0.8, 0.2), diffuse_color: DVec3::splat(0.2)};
        ObjectAppend::append(&mut sc, Box::new(sp));
        ObjectAppend::append(&mut sc, Box::new(tri));
        let light = Light { org: DVec3::new(0., 0., 0.), inten: DVec3::new(0.88, 0.42, 0.)};
        let (res, t, b1, b2) = sc.objects[0].intersection(light, DVec3::new(0.88, 0.42, 0.));
        assert_eq!(res, true);
        assert_eq!(t, 0.4683376845365324);
        assert_eq!(b1, 0);
        assert_eq!(b2, DVec2::ZERO);
    }
    #[test]
    fn test_get_attr() {
        let mut sc = Scene::create();
        let tri = MeshTriangle { vertices: vec![Triangle { 
            v0: DVec3::ZERO, v1: DVec3::ZERO, v2: DVec3::ZERO, s0: DVec2::ZERO, s1: DVec2::ZERO, s2: DVec2::ZERO 
        }], material: crate::lib::Material::DiffuseAndGlossy, ior: 1.3, specular: crate::lib::SpecularProperties(25.0, 0.8, 0.2) };
        let light = Light { org: DVec3::new(0., 0., 0.), inten: DVec3::new(0.88, 0.42, 0.)};
        let sp = Sphere { center: DVec3::new(2., 0., 0.), radius: 1.6, radius2: 2.56, material: crate::lib::Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25.0, 0.8, 0.2), diffuse_color: DVec3::splat(0.2)};
        ObjectAppend::append(&mut sc, Box::new(sp));
        ObjectAppend::append(&mut sc, Box::new(tri));
        LightAppend::append(&mut sc, light);
        let obj1 = sc.get_obj();
        obj1[0].intersection(sc.lights[0], DVec3::new(0.88, 0.42, 0.));
        let obj2 = sc.get_obj();
        obj2[0].intersection(sc.lights[0], DVec3::new(0.88, 0.42, 0.));
    }
}
