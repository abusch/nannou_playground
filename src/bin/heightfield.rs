use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    animate: bool,
    perlin: Perlin,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model {
        animate: false,
        perlin: Perlin::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::A => model.animate = !model.animate,
        Key::S => app
            .main_window()
            .capture_frame(format!("{}.png", app.exe_name().unwrap())),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect().pad(100.0);
    draw.background().color(BLACK);

    let time = if model.animate { app.time } else { 0.0 };
    let sine = time.sin();
    let slow_sine = (time / 2.0).sin();
    let theta = (time / 3.0).sin() * PI * 2.0;

    const STEP: usize = 300;
    for i in 0..STEP {
        let x = map_range(i, 0, STEP, boundary.left(), boundary.right());
        let posx = x / 100.0 + sine;
        for j in 0..STEP {
            let y = map_range(j, 0, STEP, boundary.bottom(), boundary.top());
            let posy = y / 100.0 + slow_sine;

            // rotate
            let r = vec2(posx, posy).rotate(theta).as_f64();

            // get the height from some Perlin noise
            let mut height = model.perlin.get([r.x, r.y]) as f32;
            // add a bit of higher frequency noise for more details
            height += 0.3 * model.perlin.get([r.x * 8.0, r.y * 8.0]) as f32;

            draw.ellipse()
                .x(x + y / 2.0)
                .y(y + height * 25.0)
                .w(2.0)
                .h(2.0)
                .hsva(0.5 - (height / 3.0), 0.8, 0.8, 0.3);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
