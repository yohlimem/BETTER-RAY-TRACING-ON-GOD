use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

mod circles;
mod lines;
mod rays;
mod mediums;
use circles::Circle;
use lines::Line;
use rays::Ray;
use rays::Shape;

struct Model {
    // window: Window,
    egui: Egui,
    rays: Vec<Ray>,
    shapes: Vec<Shape>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let num = 0.0;
    // let mut shapes = Vec::new();
    let mut rays = vec![];
    let ray_num = 1;
    for i in 0..ray_num {
        rays.push(Ray::new(
            // vec2(0.0, 0.0),
            vec2(150.0, -150.0),
            2.0 * PI / ray_num as f32 * i as f32,
        ));
    }
    let shapes = vec![
        Shape::Line(Line::from(vec2(100.0, -100.0), vec2(150.0, 150.0), 1.0)),
        Shape::Line(Line::from(vec2(-100.0, -100.0), vec2(-150.0, 150.0), 1.0)),
        Shape::Line(Line::from(vec2(-100.0, 100.0), vec2(100.0, 100.0), 1.0)),
        Shape::Line(Line::from(vec2(-100.0, -100.0), vec2(100.0, -110.0), 1.0)),
        Shape::Circle(Circle::from(vec2(100.0, 0.0), 50.0)),
        // Shape::Medium(mediums::Medium::new(vec2(-500.0, -100.0), vec2(500.0, 100.0), 1.5, rgba(0.0, 0.0, 1.0, 0.5))),
        // Shape::Medium(mediums::Medium::new(vec2(-500.0, -100.0), vec2(-300.0, -500.0), 1.5, rgba(0.0, 0.0, 1.0, 0.5))),
        // Shape::Medium(mediums::Medium::new(vec2(500.0, -100.0), vec2(300.0, -500.0), 1.5, rgba(0.0, 0.0, 1.0, 0.5))),
    ];

    Model { egui, rays, shapes }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        ui.label("res");
    });

    for ray in model.rays.iter_mut() {
        ray.ray_trace_loop(10, &model.shapes);
        ray.start_direction = (app.mouse.position() - ray.start_position).angle();
        // ray.start_position = app.mouse.position();
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for ray in &model.rays {
        ray.show(&draw);
    }
    for shape in &model.shapes {
        match shape {
            Shape::Line(line) => {
                draw.line()
                    .start(line.start)
                    .end(line.end)
                    .weight(1.0)
                    .color(BLACK);
            }
            Shape::Circle(circle) => {
                draw.ellipse()
                    .x_y(circle.pos.x, circle.pos.y)
                    .radius(circle.radius)
                    .color(BLACK);
            }
            Shape::Medium(medium) => {
                medium.show(&draw);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
