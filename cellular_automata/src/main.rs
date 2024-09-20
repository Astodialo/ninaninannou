use nannou::prelude::*;

const W: usize = 80;
const R: usize = 80;
const SCALE: usize = 8;
const RULE: i32 = 30;

struct Model {
    cells: Vec<Vec<i32>>,
    rows: usize,
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
        .build()
        .unwrap();

    let mut cells = [0; W + 1];
    cells[W / 2] = 1;

    Model {
        cells: [cells.to_vec()].to_vec(),
        rows: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
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
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    for y in 0..model.rows {
        for x in (0..(model.cells[y].len() * W as usize)).step_by(W / SCALE) {
            let i = (x / (W / SCALE)) % model.cells[y].len();
            draw.rect()
                .x_y(
                    x as f32 + win.left() + (W / SCALE) as f32,
                    (win.top() - (W / SCALE) as f32) - ((W * y) / SCALE) as f32,
                )
                .w_h((W / SCALE) as f32, (W / SCALE) as f32)
                .color(hsl(
                    model.cells[y][i] as f32,
                    model.cells[y][i] as f32,
                    model.cells[y][i] as f32,
                ));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
