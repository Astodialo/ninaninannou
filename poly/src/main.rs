use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let radius = 250.0;

    draw.background().color(PLUM);

    // Sine waves
    for j in (10..=((radius + 1.0) as i32)).step_by(12) {
        let points = (0..(radius + 1.0) as i32).map(|i| {
            let x = i as f32 - 75.0;
            let point = pt2(x, x.sin()) * (j as f32);
            (point, STEELBLUE)
        });

        draw.polyline().weight(3.0).points_colored(points);
    }

    // polys

    for j in (1..=((radius + 1.0) as i32)).step_by(12) {
        let points = (0..=360).step_by(j.try_into().unwrap()).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * radius;
            let y = radian.cos() * radius;
            (pt2(x, y), STEELBLUE)
        });

        draw.polyline().weight(3.0).points_colored(points);
    }

    draw.to_frame(app, &frame).unwrap()
}
