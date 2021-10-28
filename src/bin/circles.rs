use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    nannou::app(model).update(update).run();
}

struct Palette {
    colors: Vec<Rgb>,
}

impl Palette {
    fn new() -> Self {
        Palette {
            colors: vec![
                // Tango color palette
                rgb(0.988, 0.909, 0.310),
                rgb(0.930, 0.837, 0.000),
                rgb(0.768, 0.627, 0.000),
                rgb(0.545, 0.886, 0.204),
                rgb(0.455, 0.824, 0.086),
                rgb(0.304, 0.604, 0.024),
                rgb(0.988, 0.690, 0.244),
                rgb(0.960, 0.480, 0.000),
                rgb(0.808, 0.364, 0.000),
                rgb(0.446, 0.623, 0.812),
                rgb(0.204, 0.395, 0.644),
                rgb(0.125, 0.287, 0.529),
                rgb(0.678, 0.498, 0.657),
                rgb(0.460, 0.314, 0.482),
                rgb(0.362, 0.208, 0.400),
                rgb(0.914, 0.729, 0.432),
                rgb(0.757, 0.493, 0.067),
                rgb(0.560, 0.348, 0.008),
                rgb(0.937, 0.161, 0.161),
                rgb(0.800, 0.000, 0.000),
                rgb(0.644, 0.000, 0.000),
                /* rgb(0.933, 0.933, 0.925),
                rgb(0.827, 0.843, 0.811),
                rgb(0.729, 0.741, 0.713),
                rgb(0.533, 0.541, 0.521),
                rgb(0.333, 0.341, 0.325),
                rgb(0.180, 0.204, 0.212), */
            ],
        }
    }

    pub fn random(&self) -> Rgb {
        let i = random_range(0, self.colors.len());
        self.colors[i]
    }
}

struct Circle {
    o: Vec2,
    r: f32,
    c: Rgba,
}

struct Model {
    circles: Vec<Circle>,
    palette: Palette,
}

impl Model {
    fn new() -> Self {
        Model {
            circles: vec![],
            palette: Palette::new(),
        }
    }

    pub fn reset(&mut self) {
        self.circles.clear();
        self.pack_circles(80.0, 1);
        self.pack_circles(50.0, 10);
        self.pack_circles(20.0, 50);
        self.pack_circles(15.0, 20);
        self.pack_circles(10.0, 100);
        self.pack_circles(8.0, 300);
        self.pack_circles(5.0, 800);
        self.pack_circles(2.0, 5000);
        self.pack_circles(1.0, 5000);
    }

    fn pack_circles(&mut self, r: f32, max_circles: u32) {
        let max_x = WIDTH as f32 / 2.0;
        let max_y = HEIGHT as f32 / 2.0;
        // How many times we try to place a circle before bailing out
        let max_failures = 20000;
        let mut failures = 0;
        let mut num_circles = 0;
        while failures < max_failures && num_circles < max_circles {
            let x = random_range(-(max_x - r), max_x - r);
            let y = random_range(-(max_y - r), max_y - r);
            let o = vec2(x, y);
            if self
                .circles
                .iter()
                .any(|c: &Circle| o.distance(c.o) < r + c.r)
            {
                // we intersect another circle..
                failures += 1;
            } else {
                let c = self.palette.random().into();
                self.circles.push(Circle { o, r, c });
                num_circles += 1;
            }
        }
        println!("Added {} circles with radius {}", num_circles, r);
    }

    // Try to grow each circle until they touch another one
    pub fn grow(&mut self) {
        let n = self.circles.len();
        for i in 0..n {
            let c = &self.circles[i];
            let d = self
                .circles
                .iter()
                .enumerate()
                .map(|(j, c1)| {
                    if i != j {
                        c.o.distance(c1.o) - c.r - c1.r
                    } else {
                        f32::MAX
                    }
                })
                .reduce(f32::min)
                .unwrap();
            let mut c = &mut self.circles[i];
            c.r += d;
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let mut model = Model::new();

    model.reset();

    model
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.reset();
        }
        Key::G => {
            model.grow();
        }
        Key::S => {
            app.main_window().capture_frame(format!("{}.png", app.exe_name().unwrap()));
        }
        _ => (),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(0.95);
    draw.background().color(FLORALWHITE);

    for circle in &model.circles {
        draw.ellipse()
            .xy(circle.o)
            .w_h(circle.r * 2.0, circle.r * 2.0)
            .color(circle.c);
    }

    draw.to_frame(app, &frame).unwrap();
}
