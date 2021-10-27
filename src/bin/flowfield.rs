use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};
use ndarray::Array2;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    field: Array2<f32>,
    num_steps: usize,
    step_length: f32,
    frame_count: u64,
}

fn model(app: &App) -> Model {
    let perlin = Perlin::new();
    let _window = app
        .new_window()
        .size(800, 800)
        .title(app.exe_name().unwrap())
        .view(view)
        .key_pressed(key_pressed)
        .resized(window_resized)
        .build();

    let mut field = ndarray::Array2::zeros((WIDTH, HEIGHT));
    field.indexed_iter_mut().for_each(|((x, y), a)| {
        let px = x as f64 * 0.005;
        let py = y as f64 * 0.005;
        // Generate a random angle from 0 to 2*PI
        *a = TAU * perlin.get([px as f64, py as f64]) as f32;
    });
    Model {
        field,
        num_steps: 50,
        step_length: 1.0,
        frame_count: 0,
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Left => {
            if model.step_length > 0.1 {
                model.step_length -= 0.1
            }
        }
        Key::Right => model.step_length += 0.1,
        Key::Down => {
            if model.num_steps > 1 {
                model.num_steps -= 1
            }
        }
        Key::Up => model.num_steps += 1,
        Key::S => app.main_window().capture_frame("flowfield.png"),
        _ => {}
    }
}

fn window_resized(_app: &App, model: &mut Model, _: Vec2) {
    model.frame_count = 0;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.frame_count += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.window_rect();
    let boundary = window.pad(20.0);

    let wdraw = app.draw();
    // wdraw.rect().wh(window.wh()).rgba(0.0, 0.0, 0.0, 0.001);

    // Puts (0,0) in the top-left corner and y pointing down
    let draw = wdraw
        .x_y(-boundary.w() / 2.0, boundary.h() / 2.0)
        .scale_x(boundary.w() / WIDTH as f32)
        .scale_y(-boundary.h() / WIDTH as f32);
    if model.frame_count == 1 {
        wdraw.background().color(FLORALWHITE);
        draw.background().color(FLORALWHITE);
    }

    /* model.field.indexed_iter().for_each(|((x, y), a)| {
        draw.arrow()
            .weight(0.2)
            .points(pt2(x as f32, y as f32), pt2(x as f32 + a.sin() * 0.8, y as f32 + a.cos() * 0.8))
            .hsv(a / TAU, 0.8, 0.8);
    }); */
    /* let mut x: f32 = random_range(100.0, 400.0);
    let mut y: f32 = random_range(100.0, 400.0); */
    let mut x = random_range(0.0, WIDTH as f32);
    let mut y = random_range(0.0, HEIGHT as f32);

    let mut pts = Vec::with_capacity(model.num_steps);
    for _ in 0..model.num_steps {
        let px = x.floor().clamp(0.0, (WIDTH - 1) as f32) as usize;
        let py = y.floor().clamp(0.0, (HEIGHT - 1) as f32) as usize;
        let angle = model.field[(px, py)];
        pts.push(vec2(x, y));
        // draw.ellipse().x_y(x, y).w_h(0.5, 0.5).hsv(angle / TAU, 0.8, 0.8);
        x += angle.sin() * model.step_length;
        y += angle.cos() * model.step_length;
    }

    draw.path()
        .stroke()
        .stroke_weight(0.5)
        .hsva(0.0, 0.0, 0.05, 0.4)
        .join_round()
        .caps_round()
        .points(pts);

    draw.to_frame(app, &frame).unwrap();
}
