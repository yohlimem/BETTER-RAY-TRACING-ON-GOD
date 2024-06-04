use std::fmt::format;

use nannou::prelude::*;
use nannou_egui::egui::InnerResponse;
use nannou_egui::{self, egui, Egui};

mod circles;
mod lines;
mod rays;
mod mediums;
use circles::Circle;
use lines::Line;
use rays::Ray;
use rays::Shape;

#[derive(Clone, Copy, Debug, PartialEq)]
enum State{
    LightBulb,
    SingleRay,
    FOV,
}
struct Model {
    // window: Window,
    egui: Egui,
    rays: Vec<Ray>,
    shapes: Vec<Shape>,
    ray_num: usize,
    state: State,
    last_state: State,
    fov: f32,
    
}
// TODO: https://www.youtube.com/watch?v=naaeH1qbjdQ
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
    let ray_num = 50;
    for i in 0..ray_num {
        let angle = 2.0 * PI / ray_num as f32 * i as f32;
        rays.push(Ray::new(
            // vec2(0.0, 0.0),
            vec2(150.0, -150.0),
            vec2(angle.cos(), angle.sin()),
            vec2(angle.cos(), angle.sin()),
        ));
    }
    let shapes = vec![
        Shape::Line(Line::from(vec2(100.0, -100.0), vec2(150.0, 150.0), 1.0)),
        Shape::Line(Line::from(vec2(-150.0, 150.0), vec2(-100.0, -100.0), 1.0)),
        // Shape::Line(Line::from(vec2(-100.0, 100.0), vec2(100.0, 100.0), 1.0)),
        Shape::Line(Line::from(vec2(-100.0, -100.0), vec2(100.0, -110.0), 1.0)),
        Shape::Circle(Circle::from(vec2(100.0, 0.0), 50.0)),
        Shape::Medium(mediums::Medium::new(vec2(-500.0, -100.0), vec2(500.0, 100.0), 1.5, rgba(0.0, 0.0, 1.0, 0.5))),
        // Shape::Medium(mediums::Medium::new(vec2(-300.0, -500.0), vec2(-500.0, -100.0), 1.5, rgba(0.0, 0.0, 1.0, 0.5))),
        // Shape::Medium(mediums::Medium::new(vec2(300.0, -500.0), vec2(500.0, -100.0), 1.5, rgba(0.0, 0.0, 1.0, 0.5))),
    ];

    Model { egui, rays, shapes, state: State::FOV, last_state: State::LightBulb, ray_num, fov: 50.0}
}

fn update(app: &App, model: &mut Model, update: Update) {
    {
        let egui = &mut model.egui;
        egui.set_elapsed_time(update.since_start);

        let ctx = egui.begin_frame();

        egui::Window::new("Rum window").show(&ctx, |ui| {
            ui.label("controls");
            let ray_num_number = ui.add(egui::Slider::new(&mut model.ray_num, 0..=100).text("Rays"));
            if model.state == State::FOV {
                let fov_change = ui.add(egui::Slider::new(&mut model.fov, 0.0..=180.0).text("FOV"));
                if fov_change.changed() {
                    // println!("changed fov");
                    model.rays = update_ray_state(&model.rays, model.state, model.ray_num, model.fov, app);

                }
            }
            if ray_num_number.changed() {
                model.rays = update_ray_state(&model.rays, model.state, model.ray_num, model.fov, app);

            }
            egui::ComboBox::from_label("Choose state").selected_text(format!("{:?}", model.state))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut model.state, State::LightBulb, "LightBulb");
                ui.selectable_value(&mut model.state, State::SingleRay, "SingleRay");
                ui.selectable_value(&mut model.state, State::FOV, "FOV");
            });
        });
    }
    if model.state != model.last_state {
        model.last_state = model.state;
        model.rays = update_ray_state(&model.rays, model.state, model.ray_num, model.fov, app);
    }
    



    

    for ray in model.rays.iter_mut() {
        ray.ray_trace_loop(10, &model.shapes);
        if model.state == State::SingleRay{
            ray.start_direction = (app.mouse.position() - ray.start_position).normalize();
        } else if model.state == State::FOV {
            let angle = (app.mouse.position() - ray.start_position).angle() + ray.offset.angle();
            ray.start_direction = vec2(angle.cos(), angle.sin());
        }
        if app.mouse.buttons.left().is_down() {
            ray.start_position = app.mouse.position();
        }
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

fn generate_fov(ray_num: usize, start_pos: Vec2, fov: f32) -> Vec<Ray> {
    let mut rays = vec![];
    for i in 0..ray_num {
        let angle = ((i as f32 - ray_num as f32/2.0 ) / ray_num as f32) * fov;
        rays.push(Ray::new(
            start_pos,
            vec2(angle.cos(), angle.sin()),
            vec2(angle.cos(), angle.sin()),
            
        ));
    }
    rays

}

fn update_ray_state(rays_model: &Vec<Ray>, state_model: State, ray_num: usize, fov: f32, app: &App) -> Vec<Ray>{
    let mut rays = vec![];
    match state_model {
        State::LightBulb => {
            for i in 0..ray_num {
                //
                let angle = 2.0 * PI / ray_num as f32 * i as f32;
                rays.push(Ray::new(
                    // vec2(0.0, 0.0),
                    vec2(150.0, -150.0),
                    vec2(angle.cos(), angle.sin()),
                    vec2(angle.cos(), angle.sin()),
                ));
            }
            rays
        }
        State::SingleRay => {
            let last_ray_pos = rays_model.first().unwrap().start_position;
            vec![Ray::new(last_ray_pos, (app.mouse.position() - last_ray_pos).normalize(), Vec2::ZERO)]
        }
        State::FOV => {
            let last_ray_pos = rays_model.first().unwrap().start_position;

            generate_fov(ray_num, last_ray_pos, deg_to_rad(fov))
        }
    }
}