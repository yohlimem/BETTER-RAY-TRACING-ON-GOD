use nannou::prelude::*;
use crate::rays::Shape_Util;


#[derive(Clone, Copy, Debug)]
pub struct Medium {
    pub min: Vec2,
    pub max: Vec2,
    refractive_index: f32,
    color: Rgba,
}

impl Medium {
    pub fn new(min: Vec2, max: Vec2, refractive_index: f32, color: Rgba) -> Self {
        Medium {
            min,
            max,
            refractive_index,
            color,
        }
    }
    pub fn show(&self, draw: &Draw) {
        draw.rect()
            .x_y((self.min.x + self.max.x) / 2.0, (self.min.y + self.max.y) / 2.0)
            .w_h(self.max.x - self.min.x, self.max.y - self.min.y)
            .color(self.color);
    }
    pub fn refractive_index(&self) -> f32 {
        self.refractive_index
    }
    pub fn color(&self) -> Rgba {
        self.color
    }

    pub fn calculate_refractive_angle(&self, n1: f32, enter_angle: f32,) -> f32{
        let refractive_angle = (n1 * (enter_angle).sin() / self.refractive_index).asin();
        if refractive_angle.is_nan() {
            return enter_angle;
        }
        refractive_angle
    }
    pub fn calculate_refractive_angle_two_mediums(n1:f32, n2: f32, enter_angle: f32,) -> f32{
        let refractive_angle = (n1 * (enter_angle).sin() / n2).asin();
        if refractive_angle.is_nan() {
            return enter_angle;
        }
        refractive_angle
    }
}

impl Shape_Util for Medium {
    fn intersect(&self, point: &Vec2) -> bool {
            point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
    fn compare(&self, medium: &Medium) -> bool {
        self.min == medium.min && self.max == medium.max
    }
}