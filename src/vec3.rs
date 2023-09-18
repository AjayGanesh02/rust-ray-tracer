use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn newz() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}
