use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    a: u64,
    b: u64,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .view(view)
        .key_pressed(key_pressed)
        .build();
    Model { a: 1, b: 2 }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Left => {
            if model.a > 1 {
                model.a -= 1
            }
        }
        Key::Right => model.a += 1,
        Key::Down => {
            if model.b > 1 {
                model.b -= 1
            }
        }
        Key::Up => model.b += 1,
        _ => {}
    }
}
fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.window_rect();
    let boundary = window.pad(20.0);

    let draw = app.draw();
    let delta = (app.elapsed_frames() % 300) as f32 / 300.0 * TAU;

    draw.rect().wh(window.wh()).rgba(0.0, 0.0, 0.0, 1.0);
    let max_t = TAU / gcd(model.a, model.b) as f32;

    const NUM_STEPS: usize = 10000;
    let mut pts = Vec::with_capacity(NUM_STEPS);
    for i in 0..=NUM_STEPS {
        let t = map_range(i, 0, NUM_STEPS, 0.0, max_t);
        let x = (model.a as f32 * t + delta).sin() * boundary.right();
        let y = (model.b as f32 * t).sin() * boundary.top();
        pts.push(vec2(x, y));
    }

    draw.path()
        .stroke()
        .stroke_weight(1.0)
        .hsva(0.66, 0.8, 0.8, 0.8)
        .join_round()
        .caps_round()
        .points(pts);

    let s = format!("a={}, b={}", model.a, model.b);
    draw.text(&s)
        .xy(boundary.mid_top())
        .font_size(18)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else if a < b {
        gcd(a, b % a)
    } else {
        gcd(b, a % b)
    }
}
