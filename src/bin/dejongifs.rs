use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct DeJong {
    pts: Vec<Point2>,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl DeJong {
    fn new(pts: Vec<Point2>, a: f32, b: f32, c: f32, d: f32) -> Self {
        DeJong { pts, a, b, c, d }
    }

    pub fn iterate(&mut self) {
        let (a, b, c, d) = (self.a, self.b, self.c, self.d);
        self.pts.iter_mut().for_each(|p| {
            let new_p = pt2(
                (a * p.y).sin() - (b * p.x).cos(),
                (c * p.x).sin() - (d * p.y).cos(),
            );
            *p = new_p;
        });
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point2> {
        self.pts.iter()
    }
}

struct Model {
    ifs: DeJong,
    frame_count: u64,
}

fn generate_random_points(n: usize) -> Vec<Point2> {
    std::iter::from_fn(|| Some(pt2(random_range(-2.0, 2.0), random_range(-2.0, 2.0))))
        .take(n)
        .collect()
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(800, 800)
        .view(view)
        .resized(window_resized)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let pts = generate_random_points(1000);
    Model {
        ifs: DeJong::new(pts, 0.97, -1.90, 1.38, -1.50),
        frame_count: 0,
    }
}

fn update(_app: &App, model: &mut Model, _event: Update) {
    model.frame_count += 1;
    model.ifs.iterate();
}

fn window_resized(_app: &App, model: &mut Model, _: Vec2) {
    model.frame_count = 0;
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.frame_count = 0;
            model.ifs.a = random_range(-PI, PI);
            model.ifs.b = random_range(-PI, PI);
            model.ifs.c = random_range(-PI, PI);
            model.ifs.d = random_range(-PI, PI);
            model.ifs.pts = generate_random_points(1000);
        }
        Key::A => {
            model.frame_count = 0;
            model.ifs.a = random_range(-PI, PI);
            model.ifs.pts = generate_random_points(1000);
        }
        Key::B => {
            model.frame_count = 0;
            model.ifs.b = random_range(-PI, PI);
            model.ifs.pts = generate_random_points(1000);
        }
        Key::C => {
            model.frame_count = 0;
            model.ifs.a = random_range(-PI, PI);
            model.ifs.pts = generate_random_points(1000);
        }
        Key::D => {
            model.frame_count = 0;
            model.ifs.d = random_range(-PI, PI);
            model.ifs.pts = generate_random_points(1000);
        }
        Key::S => app
            .main_window()
            .capture_frame(format!("{}.png", app.exe_name().unwrap())),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(190.0);
    if model.frame_count == 1 {
        draw.background().color(FLORALWHITE);
    }

    model.ifs.iter().for_each(|p| {
        draw.ellipse()
            .xy(*p)
            .w_h(0.005, 0.005)
            .color(hsva(0.0, 0.0, 0.0, 0.2));
    });

    draw.to_frame(app, &frame).unwrap();
}
