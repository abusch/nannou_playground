use std::f32::consts::FRAC_PI_4;

use nannou::{Ui, prelude::*, rand::{prelude::StdRng, Rng, SeedableRng}, ui::{Labelable, Positionable, Sizeable, Widget, color::{self, DARK_CHARCOAL}, widget, widget_ids}};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

struct Model {
    ui: Ui,
    ids: Ids,
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

widget_ids! {
    struct Ids {
        randomize,
    }
}
fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .view(view)
        .key_pressed(key_pressed)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(300, 200)
        .view(ui_view)
        .event(ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let mut ui = app.new_ui().window(ui_window).build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    ui.clear_with(DARK_CHARCOAL);
    let mut theme = ui.theme_mut();
    theme.label_color = color::WHITE;
    theme.shape_color = color::CHARCOAL;

    let mut stones = vec![];
    for y in 0..ROWS {
        for x in 0..COLS {
            stones.push(Stone::new(x as f32, y as f32));
        }
    }

    Model {
        ui,
        ids,
        seed: 0,
        disp_adj: 1.0,
        rot_adj: 1.0,
        gravel: stones,
    }
}

fn update(_app: &App, model: &mut Model, _event: Update) {
    let mut rng = StdRng::seed_from_u64(model.seed);

    for stone in model.gravel.iter_mut() {
        // factor that goes from 0 to 1 as we go down the y direction. We use that to increase
        // the amount of randomness as we go down.
        let factor = stone.y as f32 / ROWS as f32;
        let x_offset = model.disp_adj * factor * rng.gen_range(-0.5..0.5);
        let y_offset = model.disp_adj * factor * rng.gen_range(-0.5..0.5);
        let rotation = model.rot_adj * factor * rng.gen_range(-FRAC_PI_4..FRAC_PI_4);
        stone.x_offset = x_offset;
        stone.y_offset = y_offset;
        stone.rotation = rotation;
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.seed = random();
        }
        Key::Up => {
            model.disp_adj += 0.1;
        }
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => {
            model.rot_adj += 0.1;
        }
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _ => {}
    }
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
            .x_y(
                stone.x as f32 + stone.x_offset,
                stone.y as f32 + stone.y_offset,
            )
            .rotate(stone.rotation);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn ui_event(_app: &App, model: &mut Model, _event: WindowEvent) {
    let mut ui = model.ui.set_widgets();

    for _click in widget::Button::new()
        .middle()
        .w_h(125.0, 40.0)
        .label("Randomize")
        .set(model.ids.randomize, &mut ui)
    {
        model.seed = random_range(0, 1000000);
    }
}

fn ui_view(app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame_if_changed(app, &frame).unwrap();
}
