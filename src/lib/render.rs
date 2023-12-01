use std::{f64::consts::PI, mem::swap, io::{self, Write}, fs::File, usize};
use glam::{DVec3, DVec2};
use rand::Rng;

use super::{Object, Light, Scene, Material};

pub struct HitPayload {
    pub tnear: f64,
    pub idx: usize,
    pub uv: DVec2,
    pub hit_obj: Box<dyn Object>,
}
pub fn deg2rad(deg: f64) -> f64 {
    deg * PI / 180.0
}
pub fn reflect(iv: DVec3, n: DVec3) -> DVec3 {
    // iv + ov = 2k * n;
    // dot(iv, n) = |n| * k = k
    // ov = iv - 2 * dot(iv, n) * n;
    iv - 2. * iv.dot(n) * n
}
pub fn refract(iv: DVec3, n: DVec3, ior: f64) -> DVec3 {
    // 
    let mut cosi = iv.dot(n).clamp(-1., 1.);
    let mut etai = 1.;let mut etat = ior;
    let mut nx = n;
    if cosi < 0. {
        cosi = -cosi;
    } else {
        swap(&mut etai, &mut etat);
        nx = -n;
    }
    let eta = etai / etat;
    let k = 1. - eta * eta * (1. - cosi * cosi);
    match k < 0. {
        true => DVec3::ZERO,
        _ => eta * iv + (eta * cosi - k.sqrt()) * nx,
    }
}
pub fn fresnel(iv: DVec3, n: DVec3, ior: f64) -> f64 {
    let cosi = iv.dot(n).clamp(-1., 1.);
    let mut etai = 1.;let mut etat = ior;
    if cosi > 0. {
        swap(&mut etai, &mut etat);
    }
    let sint = etai / etat * 0f64.max(1. - cosi * cosi).sqrt();
    match sint >= 1. {
        true => 1.,
        false => {
            let cost = (1. - sint * sint).max(0.).sqrt();
            let ci = cosi.abs();
            let rs  = ((etat * ci) - (etai * cost)) / ((etat * ci) + (etai * cost));
            let rp = ((etai * ci) - (etat * cost)) / ((etai * ci) + (etat * cost));
            (rs * rs + rp * rp) / 2.
        }
    }
}
pub fn trace(light: Light, dir: DVec3, objects: &Vec<Box<dyn Object>>) -> Option<HitPayload> {
    let tnear = f64::MAX;
    let mut payload = None;
    objects.into_iter().for_each(|obj| {
        let (resk, tk, idxk, uvk) = obj.intersection(light, dir);
        if resk && tk < tnear {
            let _res = payload.insert(HitPayload {
                tnear: tk,
                idx: idxk,
                uv: uvk,
                hit_obj: obj.clone(),
            });
        }
    });
    payload
}
#[allow(dead_code)]
pub fn get_random_float() -> f64 {
    let mut rng = rand::thread_rng();
    let res: f64 = rng.gen();
    res
    // todo!()
}
pub fn update_progress(progress: f64) {
    let bar_width = 50;
    print!("[");
    let pos = (bar_width as f64 * progress + 1.) as i32;
    (0..bar_width).for_each(|i| {
        if i < pos {
            print!("=");
        } else if i == pos {
            print!(">");
        } else {
            print!(" ");
        }
    });
    print!("]{}%\r", (progress * 100. + 1.) as i32);
    io::stdout().flush().unwrap();
}
pub fn cast_ray(light: Light, dir: DVec3, scene: &mut Scene, depth: i32) -> DVec3 {
    if depth > scene.max_depth.into() {
        return DVec3::new(0., 0., 0.);
    }
    let mut hit_color = scene.background_color;
    if let Some(payload) = trace(light, dir, scene.get_obj()) {
        let hit_point = light.org + dir * payload.tnear;
        let (n, st) = payload.hit_obj.get_surface_properties(hit_point, dir, payload.idx, payload.uv);
        match payload.hit_obj.get_material_properties() {
            Material::ReflectionAndRefraction => {
                let reflect_dir = reflect(dir, n).normalize();
                let refract_dir = refract(dir, n, payload.hit_obj.get_ior()).normalize();
                let reflect_ray_org = match reflect_dir.dot(n) < 0. {
                    true => hit_point - n * scene.epsilon,
                    false => hit_point + n * scene.epsilon,
                };
                let refract_ray_org = match reflect_ray_org.dot(n) < 0. {
                    true => hit_point - n * scene.epsilon,
                    false => hit_point + n * scene.epsilon,
                };
                let reflect_color = cast_ray(Light { org: reflect_ray_org, inten: light.inten }, reflect_dir, scene, depth + 1);
                let refract_color = cast_ray(Light { org: refract_ray_org, inten: light.inten }, refract_dir, scene, depth + 1);
                let kr = fresnel(dir, n, payload.hit_obj.get_ior());
                hit_color = reflect_color * kr + refract_color * (1. - kr);
            },
            Material::Reflection => {
                let kr = fresnel(dir, n, payload.hit_obj.get_ior());
                let reflect_dir = reflect(dir, n);
                let reflect_ray_org = match reflect_dir.dot(n) < 0. {
                    true => hit_point + n * scene.epsilon,
                    false => hit_point - n * scene.epsilon,
                };
                hit_color = cast_ray(Light { org: reflect_ray_org, inten: light.inten }, reflect_dir, scene, depth + 1) * kr;
            },
            _ => {
                let mut light_amt = DVec3::ZERO;let mut specular_color = DVec3::ZERO;
                let shadow_org = match dir.dot(n) < 0. {
                    true => hit_point + n * scene.epsilon,
                    false => hit_point - n * scene.epsilon,
                };
                scene.get_light().iter().for_each(|li| {
                    let mut light_dir = li.org - hit_point;
                    let light_distance = light_dir.dot(light_dir);
                    light_dir = light_dir.normalize();
                    let ldn = light_dir.dot(n).max(0.);
                    let shadow_res = trace(Light { org: shadow_org, inten: light.inten }, light_dir, scene.get_obj());
                    light_amt += match shadow_res.is_some() && (shadow_res.unwrap().tnear.powf(2.) < light_distance) {
                        true => DVec3::ZERO,
                        false => li.inten * ldn,
                    };
                    let reflect_dir = reflect(-light_dir, n);
                    specular_color += f64::powf(-reflect_dir.dot(dir).max(0.), payload.hit_obj.get_specular_properties().0) * li.inten;
                    hit_color = light_amt * payload.hit_obj.eval_diffuse_color(st) * payload.hit_obj.get_specular_properties().1 + specular_color * payload.hit_obj.get_specular_properties().2;
                })
            }
        }
    }
    hit_color
}
pub fn render(scene: &mut Scene) {
    let is: usize = (scene.width * scene.height) as usize;
    let mut frame_buffer: Vec<DVec3> = vec![DVec3::ZERO; is];
    let img_rto = scene.width as f64 / scene.height as f64;
    let scale = deg2rad(scene.fov * 0.5).tan();
    let eye_pos = DVec3::ZERO;
    let mut m: usize = 0;
    for j in 0..scene.height as usize {
        for i in 0..scene.width as usize {
            let x = ((i as f64 + 0.5) * 2. / scene.width as f64 - 1.) * scale * img_rto;
            let y = ((j as f64 + 0.5) * 2. / scene.height as f64 - 1.) * -scale;
            let dir = DVec3::new(x, y, -1.).normalize(); 
            //camera org: 0,0,0   dir = (x,y,-1).normalize()
            frame_buffer[m] = cast_ray(Light {org: eye_pos, inten: DVec3::ZERO}, dir, scene, 0);
            m += 1;
        }
        update_progress(j as f64 / scene.height as f64);
    }
    println!("");
    let mut fp = match File::create("binary.ppm") {
        Ok(fp) => fp,
        Err(e) => panic!("Failed to create file: {}", e),
    };
    let s = format!("P6\n{} {}\n255\n", scene.width, scene.height);
    fp.write_all(s.as_bytes()).unwrap();
    frame_buffer.iter().for_each(|v| {
        let ss: &[u8] = &[(v.x.clamp(0., 1.) * 255.) as u8, (v.y.clamp(0., 1.) * 255.) as u8, (v.z.clamp(0., 1.) * 255.) as u8];
        match fp.write_all(ss) {
            Ok(_) => {},
            Err(_) => panic!("Failed to write frame_buffer"),
        };
    })
    
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use glam::{DVec3, DVec2};

    use crate::lib::{Scene, MeshTriangle, Triangle, Sphere, ObjectAppend, Light, SpecularProperties, Material, LightAppend};

    use super::{trace, render};

    #[test]
    fn test_trace() {
        let mut sc = Scene::create();
        let tri = MeshTriangle { vertices: vec![Triangle { 
            v0: DVec3::ZERO, v1: DVec3::ZERO, v2: DVec3::ZERO, s0: DVec2::ZERO, s1: DVec2::ZERO, s2: DVec2::ZERO 
        }], material: crate::lib::Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25.0, 0.8, 0.2)};
        let sp = Sphere { center: DVec3::new(2., 0., 0.), radius: 1.6, radius2: 2.56, material: crate::lib::Material::DiffuseAndGlossy, ior: 1.3, specular: SpecularProperties(25.0, 0.8, 0.2), diffuse_color: DVec3::splat(0.2)};
        ObjectAppend::append(&mut sc, Box::new(sp));
        ObjectAppend::append(&mut sc, Box::new(tri));
        let light = Light { org: DVec3::new(0., 0., 0.), inten: DVec3::new(0.88, 0.42, 0.)};
        let dir = DVec3::new(0.88, 0.42, 0.);
        let payload = trace(light, dir, sc.get_obj());
        dbg!(payload.is_some());
    }
    #[test]
    fn test_render(){
        let mut sc = Scene::window(1280, 960);
        let sph1 = MeshTriangle {
            vertices: vec![
                Triangle { v0: DVec3::new(5., -3., -6.), v1: DVec3::new(5., -3., -16.), v2: DVec3::new(-5., -3., -16.), s0: DVec2::new(0.8, 0.), s1: DVec2::new(0., 0.8), s2: DVec2::new(1., 1.) }
            ],
            material: Material::DiffuseAndGlossy,
            ior: 1.3,
            specular: SpecularProperties(25.0, 0.8, 0.2),
        };
        ObjectAppend::append(&mut sc, Box::new(sph1));
        LightAppend::append(&mut sc, Light { org: DVec3::new(3., 3., 5.), inten: DVec3::splat(0.5)});
        render(&mut sc);
    }
    #[test]
    fn test_write() {
        let mut fp = File::create("test.txt").unwrap();
        let s: &[u8] = &[1, 2, 3];
        fp.write_all(s).unwrap();
    }
}