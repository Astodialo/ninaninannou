use nannou::noise::NoiseFn;
use nannou::noise::{BasicMulti, Seedable};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: BasicMulti,
    points: Vec<Vec<f32>>,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();

    Model {
        noise: BasicMulti::new().set_seed(random()),
        points: Vec::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let rect = app.window_rect();
    if model.points.len() < rect.w() as usize {
        let step = (model.points.len() + 1) as f32;
        let mut column = Vec::new();
        for y in rect.bottom() as i32..=rect.top() as i32 {
            let amplitude = model.noise.get([step as f64 / 222., y as f64 / 888.]) as f32;
            column.push(amplitude)
        }
        model.points.push(column);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let rect = app.window_rect();
    let draw = app.draw();

    let x = model.points.len() as f32 + rect.left();
    let mut y = rect.bottom();
    let column = &model.points[model.points.len() - 1];
    for amplitude in column {
        let level = map_range(*amplitude, -1., 1., 0., 1.);
        draw.ellipse().x(x).y(y).w_h(2.8, 2.8).color(hsla(
            level * 2.8,
            level * 1.2,
            level * 1.6,
            level,
        ));
        y += 1.;
    }

    draw.to_frame(app, &frame).unwrap();
}
