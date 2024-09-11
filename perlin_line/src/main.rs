use nannou::noise::{BasicMulti, Seedable};
use nannou::noise::NoiseFn;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: BasicMulti,
    points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 600)
        .view(view)
        .build()
        .unwrap();

    Model { 
        noise: BasicMulti::new().set_seed(random()),
        points: Vec::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let rect = app.window_rect();

    if model.points.len() < rect.w() as usize {
        let step = (model.points.len() + 1) as f32;
        let amplitude = model.noise.get([step as f64 / 400., 0.]) as f32;
        model.points.push(pt2(step, amplitude));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    draw.background().color(PLUM);

    for p in &model.points {
        let y = map_range(p[1], -1., 1., rect.top() - 10., rect.bottom() + 10.);
        draw.ellipse()
            .x(p[0] + rect.left())
            .y(y)
            .w_h(2.0, 2.0)
            .color(STEELBLUE);
    }

    draw.to_frame(app, &frame).unwrap()
}
