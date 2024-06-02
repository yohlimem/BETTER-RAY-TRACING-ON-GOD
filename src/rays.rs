use crate::circles::Circle;
use crate::lines::Line;
use crate::mediums::Medium;
use nannou::prelude::*;
pub struct Ray {
    pub start_position: Vec2,
    pub start_direction: f32,
    origin: Vec2,
    pub direction: f32,      // angle
    tracer: Vec2,            // the object that moves to trace
    intersect: Option<Vec2>, // the end where the object is close
    points_draw: Vec<Vec2>,
}

pub enum Shape {
    Circle(Circle),
    Line(Line),
    Medium(Medium),
}

impl Ray {
    pub fn new(origin: Vec2, direction: f32) -> Self {
        Ray {
            origin,
            start_position: origin,
            direction,
            intersect: None,
            tracer: origin,
            points_draw: vec![],
            start_direction: direction,
        }
    }

    pub fn show(&self, draw: &Draw) {
        for i in 0..self.points_draw.len() - 1 {
            draw.line()
                .start(self.points_draw[i])
                .end(self.points_draw[i + 1])
                .weight(1.0)
                .color(BLACK);
        }
    }

    pub fn touching_object(&mut self, shapes: &Vec<Shape>) -> (Option<Shape>, Option<Vec2>) {
        for shape in shapes {
            match shape {
                Shape::Line(line) => {
                    let intersected = line.intersect(&self.tracer);
                    self.intersect = Some(self.tracer);
                    if intersected {
                        return (Some(Shape::Line(*line)), Some(self.tracer));
                    }
                }
                Shape::Circle(circle) => {
                    let intersected = circle.intersect(&self.tracer);
                    if intersected {
                        self.intersect = Some(self.tracer);
                        return (Some(Shape::Circle(*circle)), Some(self.tracer));
                    }
                }
                Shape::Medium(medium) => {
                    let intersected = medium.intersect(&self.tracer);
                    if intersected {
                        self.intersect = Some(self.tracer);
                        return (Some(Shape::Medium(*medium)), Some(self.tracer));
                    }
                }
            }
        }
        return (None, Some(self.tracer));
    }

    pub fn bounce_angle(shape: &Shape, point: Vec2, pos: Vec2) -> f32 {
        // let line_vector = line.point1 - line.point2;
        match shape {
            Shape::Line(line) => {
                let ray_vector = pos - point;
                let ray_to_ground = ray_vector.angle_between(vec2(1.0, 0.0));

                // let line_equ = line.slope();
                // if line_equ.is_none(){
                //     // println!("0.0");
                //     return ray_to_ground
                // }
                let normal_m = -1.0 / line.slope().unwrap_or(f32::MAX);
                let normal_line =
                    Line::equation_to_line(normal_m, line.intercept().unwrap_or(f32::MAX));
                let normal_vector = normal_line.to_vector();

                // let normal_to_ground = normal_vector.angle_between(vec2(-1.0, 0.0));
                let line_to_ground = line.to_vector().angle_between(vec2(1.0, 0.0));
                // let line_to_ray = line.to_vector().angle_between(ray_vector);
                // let normal_to_ray = normal_vector.angle_between(ray_vector);
                // println!("ray: {}, normal: {}", ray_to_ground, normal_to_ground);
                // println!("{}", line_to_ray);

                // println!("{}", rad_to_deg(line_to_ground));
                // if rad_to_deg(ray_to_ground + line_to_ground - PI/2.0).abs() >= 180.0{
                //     return ray_to_ground + line_to_ground

                // }
                // ray_to_ground + line_to_ground - PI/2.0
                if line_to_ground >= 0.0 {
                    // println!("bad");
                    ray_to_ground - (line_to_ground) + PI
                } else {
                    ray_to_ground - line_to_ground - PI / 2.0
                }
            }
            Shape::Circle(circle) => {
                let normal = circle.normal(point);
                let ray_vector = point - pos;
                let normal_to_ray = ray_vector.angle_between(normal);
                let normal_to_x = normal.angle_between(vec2(1.0, 0.0));
                if normal_to_x < 0.0 {
                    normal_to_ray + abs(normal_to_x) + PI
                } else {
                    normal_to_ray - abs(normal_to_x) + PI
                }
                // (normal_to_ray + abs(normal_to_x))
            }
            Shape::Medium(medium) => {
                let ray_vector = pos - point;
                let ray_to_ground = ray_vector.angle_between(vec2(1.0, 0.0));
                let new_angle = medium.calculate_refractive_angle(1.0, ray_to_ground);    
                ray_to_ground + new_angle
            }
        }
    }
    pub fn ray_trace(
        &mut self,
        step: f32,
        shapes: &Vec<Shape>,
        last_shape: &Option<Shape>,
    ) -> (Option<Shape>, Vec2) {
        self.tracer = self.origin;
        let step_dir = vec2(self.direction.cos(), self.direction.sin()) * step;
        // println!("direction: {}", self.direction);
        for _ in 0..(1000.0 / step) as usize {
            self.tracer += step_dir;
            let (shape, point) = self.touching_object(shapes);
            if shape.is_some() {
                match shape {
                    Some(Shape::Line(line)) => {
                        if let Some(Shape::Line(last_line)) = last_shape {
                            if line.compare(&last_line) {
                                continue;
                            }
                        }
                    }
                    Some(Shape::Circle(circle)) => {
                        if let Some(Shape::Circle(last_circle)) = last_shape {
                            if circle.compare(&last_circle) {
                                continue;
                            }
                        }
                    }
                    Some(Shape::Medium(medium)) => {
                        if let Some(Shape::Medium(last_medium)) = last_shape {
                            if medium.compare(&last_medium) {
                                continue;
                            }
                        }
                    }
                    None => {}
                }
                return (shape, point.unwrap());
            }
        }
        return (None, self.tracer);
    }

    pub fn ray_trace_loop(&mut self, bounces: u32, shapes: &Vec<Shape>) {
        self.origin = self.start_position;
        self.tracer = self.origin;
        self.points_draw.clear();
        self.points_draw.push(self.origin);
        self.direction = self.start_direction;

        let mut last_shape = None;
        for _ in 0..bounces as usize {
            let (shape, point) = self.ray_trace(0.1, shapes, &last_shape);
            if let Some(shape) = &shape {
                let bounce_angle = Ray::bounce_angle(shape, self.tracer, self.origin);
                self.direction = bounce_angle;
            }
            last_shape = shape;
            self.points_draw.push(self.tracer);
            self.origin = point;
        }
        // println!("points_draw': {:?}", self.points_draw);
    }
}

pub trait Shape_Util {
    fn compare(&self, other: &Self) -> bool;
    fn intersect(&self, point: &Vec2) -> bool;
}
