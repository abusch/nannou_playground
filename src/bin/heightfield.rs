use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect().pad(200.0);
    draw.background().color(BLACK);
    let perlin = Perlin::new();

    let sine = app.time.sin();
    let slow_sine = (app.time / 2.0).sin();
    let theta = (app.time / 3.0).sin() * PI * 2.0;

    const STEP: usize = 150;
    for i in 0..STEP {
        let x = map_range(i, 0, STEP, boundary.left(), boundary.right());
        let posx = x / 100.0 + sine;
        for j in 0..STEP {
            let y = map_range(j, 0, STEP, boundary.bottom(), boundary.top());
            let posy = y / 100.0 + slow_sine;

            // rotate
            let r = vec2(posx, posy).rotate(theta as f32).as_f64();

            // get the height from some Perlin noise
            let mut height = perlin.get([r.x, r.y]) as f32;
            // add a bit of higher frequency noise for more details
            height += 0.3 * perlin.get([r.x * 5.0, r.y * 5.0]) as f32;

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
