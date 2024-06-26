use crate::circles::Circle;
use crate::lines::Line;
use crate::mediums::Medium;
use nannou::prelude::*;

#[derive(Clone, Debug)]
pub struct Ray {
    pub start_position: Vec2,
    pub start_direction: Vec2,
    pub offset: Vec2,
    origin: Vec2,
    pub direction: Vec2,      // normal
    tracer: Vec2,            // the object that moves to trace
    intersect: Option<Vec2>, // the end where the object is close
    points_draw: Vec<Vec2>,
}


#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Circle(Circle),
    Line(Line),
    Medium(Medium),
}

impl Ray {
    pub fn new(origin: Vec2, direction: Vec2, offset: Vec2) -> Self {
        Ray {
            origin,
            start_position: origin,
            direction,
            intersect: None,
            tracer: origin,
            points_draw: vec![],
            start_direction: direction,
            offset,
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

    pub fn bounce_angle(shape: &Shape, point: Vec2, pos: Vec2, is_leaving: bool) -> Vec2 {
        // let line_vector = line.point1 - line.point2;
        let ray_vector = point - pos;
        match shape {
            Shape::Line(line) => {

                let normal_m = -1.0 / line.slope().unwrap_or(f32::MAX);
                let normal_line = Line::equation_to_line(normal_m, line.intercept().unwrap_or(f32::MAX));
                let normal_vector = normal_line.to_vector();

                Self::reflect(ray_vector, normal_vector).normalize()
            }
            Shape::Circle(circle) => {
                let normal = circle.normal(point);
                Self::reflect(ray_vector, normal).normalize()

            }
            Shape::Medium(medium) => {
                let normal = medium.normal_at_point(point);
                let refractive_angle = if is_leaving {
                    Medium::calculate_refractive_angle_two_mediums(1.5, 1.0,ray_vector, normal)
                    
                } else {
                    Medium::calculate_refractive_angle_two_mediums(1.0, 1.5,ray_vector, -normal)
                };

                // refractive_angle.unwrap_or(Self::reflect(ray_vector, normal).normalize()).normalize()

                match refractive_angle {
                    Some(angle) => angle,
                    None => {
                        Self::reflect(ray_vector, normal).normalize()},
                }
                
                 
            }
        }
    }
    pub fn ray_trace(
        &mut self,
        step: f32,
        shapes: &Vec<Shape>,
        last_shape: &Option<Shape>,
        is_inside_medium: bool,
    ) -> (Option<Shape>, Vec2, bool) { // (shape, point_where_touch, is_leaving)
        self.tracer = self.origin;
        let step_dir = self.direction * step;
        let mut last_shape = last_shape.clone();
        // println!("{:?}", last_shape);
        for _ in 0..(1000.0 / step) as usize {
            self.tracer += step_dir;
            let (shape, point) = self.touching_object(shapes);
            if shape.is_some() {
                if let Some(Shape::Medium(_)) = shape {
                    if is_inside_medium {
                        last_shape = shape;
                        continue;
                    }
                }
                match shape.unwrap() {
                    Shape::Line(line) => {
                        if let Some(Shape::Line(last_line)) = last_shape {
                            if line.compare(&last_line) {
                                continue;
                            }
                        }
                    }
                    Shape::Circle(circle) => {
                        if let Some(Shape::Circle(last_circle)) = last_shape {

                            if circle.compare(&last_circle) {
                                continue;
                            }
                        }
                    }
                    Shape::Medium(medium) => {
                        if let Some(Shape::Medium(last_medium)) = last_shape {
                            if medium.compare(&last_medium) {
                                
                                continue;
                            }
                        }
                    }
                }
                return (shape, point.unwrap(), false);
            } else {
                // if is_inside_medium{continue;}
                if let Some(Shape::Medium(last_medium)) = last_shape {
                    // println!("leaving");
                    return (Some(Shape::Medium(last_medium)), self.tracer, true);
                }
            }
        }
        return (None, self.tracer, false);
    }

    pub fn ray_trace_loop(&mut self, bounces: u32, shapes: &Vec<Shape>) {
        self.origin = self.start_position;
        self.tracer = self.origin;
        self.points_draw.clear();
        self.points_draw.push(self.origin);
        self.direction = self.start_direction;
        let mut is_inside_medium = false;

        let mut last_shape = None;
        for _ in 0..bounces as usize {
            let (shape, point, is_leaving) = self.ray_trace(0.1, shapes, &last_shape, is_inside_medium);
            if let Some(shape) = &shape {
                let bounce_normal = Ray::bounce_angle(shape, self.tracer, self.origin, is_leaving);
                self.direction = bounce_normal;
            }
            if is_leaving {
                last_shape = None;
            } else {
                last_shape = shape;

            }
            is_inside_medium = !is_leaving;
            self.points_draw.push(self.tracer);
            self.origin = point;
            // println!("{:?}", is_leaving);
        }
    }
}

impl Ray {
    pub fn reflect(vec: Vec2, normal: Vec2) -> Vec2 {
        // https://www.youtube.com/watch?v=naaeH1qbjdQ
        vec - 2.0 * vec.dot(normal) / normal.dot(normal) * normal
    }
}

pub trait Shape_Util {
    fn compare(&self, other: &Self) -> bool;
    fn intersect(&self, point: &Vec2) -> bool;
}
