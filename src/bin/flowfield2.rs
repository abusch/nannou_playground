use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    nannou::app(model).update(update).run();
}

struct Flowfield {
    perlin: Perlin,
    z_off: f64,
}
impl Flowfield {
    pub fn new() -> Self {
        Self {
            perlin: Perlin::new(),
            z_off: 0.0,
        }
    }

    pub fn at(&self, p: Vec2) -> Vec2 {
        let px = p.x as f64 * 0.005;
        let py = p.y as f64 * 0.005;
        // Generate a random angle from 0 to 2*PI
        let angle = TAU
            * (0.7 * self.perlin.get([px, py, self.z_off])
                + 0.3 * self.perlin.get([px * 5.0, py * 5.0, self.z_off])) as f32;
        vec2(angle.sin(), angle.cos())
    }

    pub fn update(&mut self) {
        self.z_off += 0.01;
    }
}

struct Particle {
    prev_pos: Point2,
    cur_pos: Point2,
    vel: Vec2,
}

impl Particle {
    pub fn new(p: Vec2) -> Self {
        Particle {
            prev_pos: p,
            cur_pos: p,
            vel: vec2(0.0, 0.0),
        }
    }
    pub fn update_prev(&mut self) {
        self.prev_pos = self.cur_pos;
    }

    pub fn apply_force(&mut self, accel: Vec2) {
        self.vel += accel;
        // make sure the velocity doesn't grow out of control...
        self.vel = self.vel.clamp_length_max(5.0);
        self.cur_pos += self.vel;
    }

    pub fn wrap(&mut self) {
        let mut wrap = false;
        if self.cur_pos.x < 0.0 {
            wrap = true;
            self.cur_pos.x += WIDTH as f32;
        } else if self.cur_pos.x > WIDTH as f32 {
            wrap = true;
            self.cur_pos.x -= WIDTH as f32;
        }
        if self.cur_pos.y < 0.0 {
            wrap = true;
            self.cur_pos.y += HEIGHT as f32;
        } else if self.cur_pos.y > HEIGHT as f32 {
            wrap = true;
            self.cur_pos.y -= HEIGHT as f32;
        }

        if wrap {
            self.update_prev();
        }
    }
}

struct Model {
    field: Flowfield,
    accel_factor: f32,
    frame_count: u64,
    particles: Vec<Particle>,
}

impl Model {
    fn update_particles(&mut self) {
        self.particles.iter_mut().for_each(|part| {
            part.update_prev();
            part.apply_force(self.field.at(part.cur_pos) * self.accel_factor);
            part.wrap();
        });
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(800, 800)
        .title(app.exe_name().unwrap())
        .view(view)
        .key_pressed(key_pressed)
        .resized(window_resized)
        .build();

    // Create particles
    let num_part = 2000;
    let mut particles = Vec::with_capacity(num_part);
    for _ in 0..num_part {
        let p = pt2(
            random_range(0.0, WIDTH as f32),
            random_range(0.0, HEIGHT as f32),
        );
        particles.push(Particle::new(p));
    }

    Model {
        field: Flowfield::new(),
        accel_factor: 1.0,
        frame_count: 0,
        particles,
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Left => {
            if model.accel_factor > 0.1 {
                model.accel_factor -= 0.1
            }
        }
        Key::Right => model.accel_factor += 0.1,
        Key::R => model.frame_count = 0,
        Key::S => app
            .main_window()
            .capture_frame(format!("{}.png", app.exe_name().unwrap())),
        _ => {}
    }
}

fn window_resized(_app: &App, model: &mut Model, _: Vec2) {
    model.frame_count = 0;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.frame_count += 1;
    model.update_particles();
    model.field.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.window_rect();
    let boundary = window.pad(20.0);

    let wdraw = app.draw();

    // Puts (0,0) in the top-left corner and y pointing down
    let draw = wdraw
        .x_y(-boundary.w() / 2.0, boundary.h() / 2.0)
        .scale_x(boundary.w() / WIDTH as f32)
        .scale_y(-boundary.h() / WIDTH as f32);
    if model.frame_count == 1 {
        wdraw.background().color(FLORALWHITE);
        draw.background().color(FLORALWHITE);
    }

    model.particles.iter().for_each(|part| {
        draw.path()
            .stroke()
            .stroke_weight(0.5)
            .hsva(0.0, 0.0, 0.05, 0.05)
            .join_round()
            .caps_butt()
            .points(vec![part.prev_pos, part.cur_pos]);
    });

    draw.to_frame(app, &frame).unwrap();
}
