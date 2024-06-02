use nannou::{ease::circ, prelude::*};

use crate::rays::Shape_Util;
#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub pos: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            radius: 0.1,
        }
    }
    pub fn from(pos: Vec2, radius: f32) -> Self {
        Self { pos, radius }
    }
}

impl Circle {
    pub fn normal(&self, point: Vec2) -> Vec2 {
        self.pos - point
    }
}

impl Shape_Util for Circle {
    fn intersect(&self, point: &Vec2) -> bool {
        let dx = self.pos.x - point.x;
        let dy = self.pos.y - point.y;
        let distance_squared = dx * dx + dy * dy;
        let radius_squared = self.radius * self.radius;

        distance_squared <= radius_squared
    }
    fn compare(&self, circle: &Circle) -> bool {
        self.pos == circle.pos && self.radius == circle.radius
    }
}
