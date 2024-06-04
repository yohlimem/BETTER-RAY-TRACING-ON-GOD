use nannou::prelude::*;
use crate::rays::Shape_Util;

/// Remember to not switch between max and min
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

    pub fn calculate_refractive_angle(&self, n1: f32, enter_angle: f32,) -> Vec2{
        let refractive_angle = (n1 * (enter_angle).sin() / self.refractive_index).asin();
        if refractive_angle.is_nan() {
            return vec2(refractive_angle.cos(), refractive_angle.sin());
        }
        vec2(refractive_angle.cos(), refractive_angle.sin())
    }
    pub fn calculate_refractive_angle_two_mediums(
    n1: f32,
    n2: f32,
    incident: Vec2,
    normal: Vec2,
) -> Option<Vec2>{
        let incident = incident.normalize();
        let normal = normal.normalize();
        
        let cos_theta_i = incident.dot(normal);
        let sin_theta_i2 = 1.0 - cos_theta_i * cos_theta_i;
        
        let sin_theta_r2 = (n1 / n2) * (n1 / n2) * sin_theta_i2;
        
        if sin_theta_r2 > 1.0 {
            return None; // Total internal reflection
        }
        
        let cos_theta_r = (1.0 - sin_theta_r2).sqrt();
        
        let refracted_parallel = (n1 / n2) * (incident - cos_theta_i * normal);
        let refracted_perpendicular = -cos_theta_r * normal;
        
        Some(refracted_parallel + refracted_perpendicular)

    }
    pub fn normal_at_point(&self, point: Vec2) -> Vec2 {
        let distance_from_top = self.max.y - point.y;
        let distance_from_bottom = point.y - self.min.y;
        let distance_from_right = self.max.x - point.x;
        let distance_from_left = point.x - self.min.x;

        let mut largest_distance = distance_from_top;
        let mut normal = vec2(0.0, -1.0);

        if distance_from_bottom < largest_distance {
            largest_distance = distance_from_bottom;
            // println!("distance_from_bottom: {}", distance_from_bottom);
            normal = vec2(0.0, 1.0);
        }

        if distance_from_right < largest_distance {
            largest_distance = distance_from_right;
            // println!("distance_from_right: {}", distance_from_right);

            normal = vec2(-1.0, 0.0);
        }

        if distance_from_left < largest_distance {
            // println!("distance_from_left: {}", distance_from_left);

            normal = vec2(1.0, 0.0);
        }

        normal


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