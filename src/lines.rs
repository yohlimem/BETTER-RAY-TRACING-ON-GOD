use nannou::prelude::*;

use crate::rays::Shape_Util;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
}

impl Line {
    pub fn from(start: Vec2, end: Vec2, thickness: f32) -> Self {
        Line {
            start,
            end,
            thickness,
        }
    }
    pub fn new() -> Self {
        Line {
            start: Vec2::ZERO,
            end: Vec2::ZERO,
            thickness: 1.0,
        }
    }

    /// find the distance between this line and a point
    pub fn distance_to_point(&self, point: &Vec2) -> f32 {
        // Calculate the direction vector of the line
        let line_direction = vec2(self.end.x - self.start.x, self.end.y - self.start.y);
        let point_to_target = vec2(point.x - self.start.x, point.y - self.start.y);

        // Calculate the dot product of the line direction and point_to_target
        let dot_product =
            line_direction.x * point_to_target.x + line_direction.y * point_to_target.y;

        // Calculate the distance from the point to the line
        let distance = dot_product.abs() / line_direction.length();

        // Return the distance
        distance
    }
    pub fn slope(&self) -> Option<f32> {
        let dx = self.end.x - self.start.x + 0.0001;
        let dy = self.end.y - self.start.y + 0.0001;

        // Check if the line is vertical (infinite slope)
        if dx == 0.0 {
            None
        } else {
            Some(dy / dx)
        }
    }

    pub fn intercept(&self) -> Option<f32> {
        // Calculate the slope (if not vertical)
        if let Some(slope) = self.slope() {
            let intercept = self.start.y - slope * self.start.x;
            Some(intercept)
        } else {
            // Line is vertical, no intercept
            None
        }
    }
    pub fn equation_to_line(m: f32, b: f32) -> Self {
        let start_x = 1.0;
        let start_y = m * start_x + b;
        let end_x = 10.0;
        let end_y = m * end_x + b;
        Line {
            start: vec2(start_x, start_y),
            end: vec2(end_x, end_y),
            thickness: 1.0,
        }
    }
    pub fn to_vector(&self) -> Vec2 {
        self.end - self.start
    }
}

impl Shape_Util for Line {
    /// compare two lines. does not consider thickness
    fn compare(&self, line: &Line) -> bool {
        return self.start == line.start && self.end == line.end;
    }
    /// point: the point you want to find intersection with
    ///
    /// thickness: the minimum distance an intersect will trigger
    ///
    /// written by chat GPT
    fn intersect(&self, point: &Vec2) -> bool {
        let v1 = vec2(self.end.x - self.start.x, self.end.y - self.start.y);
        let v2 = vec2(point.x - self.start.x, point.y - self.start.y);

        let dot_product = v1.x * v2.x + v1.y * v2.y;
        let squared_length_v1 = v1.x * v1.x + v1.y * v1.y;

        // Calculate the t value along the line segment
        let t = dot_product / squared_length_v1;

        // Check if the intersection point is within the bounds of the line segment
        if t >= 0.0 && t <= 1.0 {
            // Calculate the point of intersection on the line
            let intersection_point = vec2(self.start.x + t * v1.x, self.start.y + t * v1.y);

            // Calculate the distance between the intersection point and the input point
            let distance = (intersection_point.x - point.x).hypot(intersection_point.y - point.y);

            return distance <= self.thickness;
        }

        false
    }
}
