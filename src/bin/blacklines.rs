use nannou::{
    ease::{expo, quad, quart},
    prelude::*,
    rand::{
        prelude::{SeedableRng, StdRng},
        Rng,
    },
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    seed: u64,
    ease_type: u8,
    factor: f32,
    alpha: f32,
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
    // app.set_loop_mode(LoopMode::loop_once());
    Model {
        seed: 0,
        ease_type: 1,
        factor: 10.0,
        alpha: 1.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Up => model.alpha += 0.1,
        Key::Down => if model.alpha > 0.1 { model.alpha -= 0.1 },
        Key::Right => model.factor += 1.0,
        Key::Left => {
            if model.factor > 1.0 {
                model.factor -= 1.0
            }
        }
        Key::Key1 => model.ease_type = 1,
        Key::Key2 => model.ease_type = 2,
        Key::Key3 => model.ease_type = 3,
        Key::Key4 => model.ease_type = 4,
        Key::Key5 => model.ease_type = 5,
        Key::Key6 => model.ease_type = 6,
        Key::Key7 => model.ease_type = 7,
        Key::Key8 => model.ease_type = 8,
        Key::Key9 => model.ease_type = 9,
        Key::Key0 => model.ease_type = 0,
        Key::R => model.seed = random(),
        Key::S => app
            .main_window()
            .capture_frame(format!("{}.png", app.exe_name().unwrap())),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app
        .draw()
        // Add a bit of margin
        .scale(0.95)
        // move origin to top left
        .x_y(-(WIDTH as f32 / 2.0), HEIGHT as f32 / 2.0)
        // flip y axis
        .scale_y(-1.0);
    draw.background().color(FLORALWHITE);
    let c = hsva(0.0, 0.0, 0.0, model.alpha);

    let mut rng = StdRng::seed_from_u64(model.seed);

    const STEPX: usize = 500;
    const STEPY: usize = 1000;
    for i in 0..STEPX {
        let x = map_range(i, 0, STEPX, 0.0, WIDTH as f32) + rng.gen_range(-1.0..1.0);
        let weight = rng.gen_range(0.5..1.5);
        let mut pts = Vec::with_capacity(STEPY);
        for j in 0..STEPY {
            let y = map_range(j, 0, STEPY, 0.0, HEIGHT as f32);
            let frac = y / HEIGHT as f32;
            let eased_frac = match model.ease_type {
                1 => expo::ease_in_out(frac, 0.0, 1.0, 1.0),
                2 => expo::ease_in(frac, 0.0, 1.0, 1.0),
                3 => expo::ease_out(frac, 0.0, 1.0, 1.0),
                4 => quad::ease_in_out(frac, 0.0, 1.0, 1.0),
                5 => quad::ease_in(frac, 0.0, 1.0, 1.0),
                6 => quad::ease_out(frac, 0.0, 1.0, 1.0),
                7 => quart::ease_in_out(frac, 0.0, 1.0, 1.0),
                8 => quart::ease_in(frac, 0.0, 1.0, 1.0),
                9 => quart::ease_out(frac, 0.0, 1.0, 1.0),
                _ => frac,
            };
            let offset_x = model.factor * eased_frac * rng.gen_range(-1.0..1.0);
            let offset_y = model.factor / 2.0 * eased_frac * rng.gen_range(-1.0..1.0);
            pts.push(pt2(x + offset_x, y + offset_y));
        }
        draw.path()
            .stroke()
            .stroke_weight(weight)
            .join_round()
            .color(c)
            .points(pts);
    }

    draw.to_frame(app, &frame).unwrap();
}
