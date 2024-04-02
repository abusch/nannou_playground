use std::f32::consts::FRAC_PI_4;

use nannou::{
    prelude::*,
    rand::{prelude::StdRng, Rng, SeedableRng},
};
use nannou_egui::{self, egui, Egui};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

struct Model {
    ui: Egui,
    seed: u64,
    disp_adj: f32,
    rot_adj: f32,
    gravel: Vec<Stone>,
}

struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
}

impl Stone {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            x_offset: 0.0,
            y_offset: 0.0,
            rotation: 0.0,
        }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);
    let window_id = app
        .new_window()
        .title(app.exe_name().unwrap())
        .view(view)
        .raw_event(raw_window_event)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    let mut stones = vec![];
    for y in 0..ROWS {
        for x in 0..COLS {
            stones.push(Stone::new(x as f32, y as f32));
        }
    }

    Model {
        ui: egui,
        seed: 0,
        disp_adj: 1.0,
        rot_adj: 1.0,
        gravel: stones,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    // UI
    model.ui.set_elapsed_time(update.since_start);
    let ctx = model.ui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add(egui::Slider::new(&mut model.disp_adj, 0.0..=5.0).text("Displacement"));
            ui.add(egui::Slider::new(&mut model.rot_adj, 0.0..=5.0).text("Rotation"));
            if ui.button("Randomize").clicked() {
                model.seed = random_range(0, 1000000);
            }
        })
    });

    let mut rng = StdRng::seed_from_u64(model.seed);
    for stone in model.gravel.iter_mut() {
        // factor that goes from 0 to 1 as we go down the y direction. We use that to increase
        // the amount of randomness as we go down.
        let factor = stone.y / ROWS as f32;
        let x_offset = model.disp_adj * factor * rng.gen_range(-0.5..0.5);
        let y_offset = model.disp_adj * factor * rng.gen_range(-0.5..0.5);
        let rotation = model.rot_adj * factor * rng.gen_range(-FRAC_PI_4..FRAC_PI_4);
        stone.x_offset = x_offset;
        stone.y_offset = y_offset;
        stone.rotation = rotation;
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // "Grid" coordinate system. Origin is at the center of the top-left square, with y going down
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(FLORALWHITE);

    for stone in &model.gravel {
        gdraw
            .rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x + stone.x_offset, stone.y + stone.y_offset)
            .rotate(stone.rotation);
    }

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(&frame).unwrap();
}
