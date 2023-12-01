use std::f64::INFINITY;

use glam::DVec3;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Light {
    pub org: DVec3,
    pub inten: DVec3,
}
#[allow(unused)]
impl Light {
    pub fn new(org: DVec3, inten: DVec3) -> Light {
        Light { org, inten }
    }
    pub fn create() -> Light {
        Light { org: DVec3::new(0., 0., 0.), inten: DVec3::new(0., 0., 0.) }
    }
    pub fn intersection(&self, v0: DVec3, v1: DVec3, v2: DVec3, dir: DVec3) -> (bool, f64, f64, f64) {
        // o + t*d = (1-u-v)*v0+u*v1+v*v2
        let od1 = v1 - v0;let od2 = v2 - v0;
        match dir.dot(od2) {
            det if det < 0. => (false, INFINITY, 0., det),
            det => {
                let s = self.org - v0;
                let s1 = dir.cross(od2);
                match s1.dot(s) {
                    b1d if b1d < 0. => (false, INFINITY, b1d, det),
                    b1d => {
                        let s2 = s.cross(od1);
                        match s2.dot(od2) {
                            td if td < 0. => (false, INFINITY, b1d, det),
                            td => {
                                let base = 1./s1.dot(od1);
                                let t = td * base;
                                let b1 = b1d * base;
                                let b2 = det * base;
                                match 1. - b1 - b2 {
                                    el if el < 0. => (false, t, b1, b2),
                                    _ => (true, t, b1, b2),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use glam::DVec3;

    use super::Light;

    #[test]
    fn test_intersection() {
        let li = Light { org:DVec3::new(0.25, 0.14, 1.1), inten: DVec3::new(0.44, 0.44, -0.02)};
        let (res, t, b1, b2) = li.intersection(DVec3::new(-1.6, -1.5, 6.2), DVec3::new(2.1, 6.4, -4.4), DVec3::new(14.4, 13.2, -2.4), DVec3::new(0.44, 0.44, -0.02));
        assert_eq!(res, true);
        assert_eq!(t, 14.811501379424634);
        assert_eq!(b1, 0.10439076224178441);
        assert_eq!(b2, 0.666803146842921);
    }
}