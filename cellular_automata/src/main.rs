use std::process;

use chrono::prelude::*;
use nannou::prelude::*;

const RULE: i32 = 110;

const W: usize = 360;
const R: usize = 360;
const SCALE: usize = 120;
const CAPTURE: bool = false;

struct Model {
    cells: Vec<Vec<i32>>,
    rows: usize,
    clr: Vec<Vec<Hsl>>,
    paused: bool,
    ctrl_key_pressed: bool,
}

fn calc_state(a: i32, b: i32, c: i32) -> i32 {
    let ruleset = format!("{:b}", RULE);
    let mut chars: Vec<char> = ruleset.chars().collect();

    while chars.len() < 8 {
        chars.insert(0, '0')
    }

    let hood = a.to_string() + &b.to_string() + &c.to_string();
    let value = usize::from_str_radix(&hood, 2).unwrap();

    chars[7 - value].to_digit(2).unwrap() as i32
}

fn main() {
    nannou::app(model).update(update).run()
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(W as u32 * 10, R as u32 * 10)
        .view(view)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let mut cells = [0; W + 1];
    cells[W / 2] = 1;

    let mut clr = Vec::new();
    for i in 0..cells.len() {
        clr.push(hsl(
            cells[i] as f32,
            if cells[i] == 0 { 0.2 } else { 0.8 },
            if cells[i] == 0 { 0.2 } else { 0.8 },
        ))
    }

    Model {
        cells: [cells.to_vec()].to_vec(),
        rows: 0,
        clr: [clr].to_vec(),
        paused: false,
        ctrl_key_pressed: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.paused {
        return;
    }

    let t = app.elapsed_frames();

    let cells = &model.cells;
    let mut next_cells: Vec<i32> = Vec::new();

    if model.rows < R {
        for i in 0..cells[model.rows].len() {
            let mut left = 0;
            let mut right = 0;
            if i == 0 {
                left = *cells[model.rows].last().unwrap()
            } else {
                left = cells[model.rows][i - 1];
            }

            if i == cells[model.rows].len() - 1 as usize {
                right = cells[model.rows][0];
            } else {
                right = cells[model.rows][i + 1];
            }
            let state = cells[model.rows][i];
            let new_state = calc_state(left, state, right);
            next_cells.push(new_state);
        }

        model.cells.push(next_cells);
        model.rows += 1;
        model.clr.push(Vec::new());
        for i in 0..model.cells[model.rows].len() {
            model.clr[model.rows].push(hsl(
                model.cells[model.rows][i] as f32 - t as f32 / 20.,
                if model.cells[model.rows][i] == 0 {
                    0.2
                } else {
                    0.8
                },
                if model.cells[model.rows][i] == 0 {
                    0.2
                } else {
                    0.8
                },
            ))
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if model.paused {
        return;
    }

    let draw = app.draw();
    let win = app.window_rect();

    for y in 0..model.rows {
        for x in (0..(model.cells[y].len() * W / SCALE as usize)).step_by(W / SCALE) {
            let i = (x / (W / SCALE)) % model.cells[y].len();
            draw.rect()
                .x_y(x as f32 + win.left(), win.top() - (y * W / SCALE) as f32)
                .w_h((W / SCALE) as f32, (W / SCALE) as f32)
                .color(model.clr[y][i]);
        }
    }

    draw.to_frame(app, &frame).unwrap();

    if CAPTURE {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}

/// React to key-presses
fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => {
            if model.ctrl_key_pressed {
                process::exit(0);
            }
        }
        Key::S => {
            let file_path = saved_image_path(app);
            app.main_window().capture_frame(file_path);
        }
        Key::Space => {
            model.paused = !model.paused;
        }
        Key::LControl => {
            model.ctrl_key_pressed = true;
        }
        _other_key => {}
    }
}

/// React to key releases
fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::LControl => {
            model.ctrl_key_pressed = false;
        }
        _other_key => {}
    }
}

/// Get the path to the next captured frame
fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("frame{:05}", frame.nth()))
        .with_extension("png")
}

/// Get the path to the next saved image
fn saved_image_path(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("saved")
        .join(format!("image{:05}", chrono::offset::Local::now()))
        .with_extension("png")
}
