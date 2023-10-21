use crate::rng;
use std::ops;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub e: [f32; 3],
}

const ZERO: Vec3 = Vec3 { e: [0.0, 0.0, 0.0] };

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
        ZERO
    }

    pub fn random() -> Vec3 {
        Vec3 {
            e: [rng::random(), rng::random(), rng::random()],
        }
    }

    pub fn random_in_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            e: [
                rng::random_in_range(min, max),
                rng::random_in_range(min, max),
                rng::random_in_range(min, max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let vec = Vec3::random_in_range(-1.0, 1.0);

            if vec.length_squared() < 1.0 {
                break vec;
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
        vec - 2.0 * vec.dot(normal) * normal
    }

    pub fn refract(vec: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min((-vec).dot(normal), 1.0);
        let r_out_perp = etai_over_etat * (vec + cos_theta * normal);
        let r_out_par = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_par
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                -(self.e[0] * other.e[2] - self.e[2] * other.e[0]),
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s: f32 = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.e[0], self * other.e[1], self * other.e[2])
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(other * self.e[0], other * self.e[1], other * self.e[2])
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-v, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_mul_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_mul_scalar() {
        let t = 3.0;
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(t * v, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_div_scalar() {
        let t = 3.0;
        let v = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(v / t, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.cross(v2), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(0.0, 0.0, 2.0);
        assert_eq!(v.length(), 2.0);
    }

    #[test]
    fn test_unit_direction() {
        let v1 = Vec3::new(10.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 20.0, 0.0);
        let v3 = Vec3::new(0.0, 0.0, 30.0);
        assert_eq!(v1.unit_vector(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(v2.unit_vector(), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(v3.unit_vector(), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_near_zero() {
        let v1 = Vec3::new(1e-9, 1e-9, 1e-9);
        let v2 = Vec3::new(1e-8, 1e-8, 1e-8);
        let v3 = Vec3::new(1e-9, -1e-9, 1e-9);
        let v4 = Vec3::new(-1e-8, 1e-8, -1e-8);
        assert_eq!(v1.near_zero(), true);
        assert_eq!(v2.near_zero(), false);
        assert_eq!(v3.near_zero(), true);
        assert_eq!(v4.near_zero(), false);
    }
}
