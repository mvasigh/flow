use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

const GAP: i32 = 13;

#[derive(Debug)]
struct FlowPoint {
    pos: Vector2<f64>,
    mag: f64,
}

impl FlowPoint {
    fn new(x: f64, y: f64, mag: f64) -> FlowPoint {
        FlowPoint {
            pos: vec2(x, y),
            mag,
        }
    }

    fn set_mag(&mut self, mag: f64) {
        self.mag = mag;
    }
}

#[derive(Debug)]
struct FlowField {
    points: Vec<FlowPoint>,
}

impl FlowField {
    fn new<F>(width: i32, height: i32, magnitude: F) -> FlowField
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut points = Vec::new();

        let max_x = ((width / 2) / GAP) + GAP;
        let min_x = -(width / 2) / GAP;
        let min_y = -(height / 2) / GAP;
        let max_y = ((height / 2) / GAP) + GAP;

        for x in min_x..max_x {
            for y in min_y..max_y {
                let px = (x * GAP) as f64;
                let py = (y * GAP) as f64;
                let fp = FlowPoint::new(px, py, magnitude(px, py));
                points.push(fp);
            }
        }

        FlowField { points }
    }

    fn update<F>(&mut self, func: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        for point in self.points.iter_mut() {
            point.set_mag(func(point.pos.x, point.pos.y))
        }
    }

    fn draw(&self, draw: &Draw) {
        for point in &self.points {
            let c = (point.mag * point.mag) as f32;
            draw.ellipse()
                .x(point.pos.x as f32)
                .y(point.pos.y as f32)
                .radius((point.mag as f32) * 8.0)
                .rgb(
                    clamp(1.0 - c, 0.6, 1.0),
                    clamp(c, 0.0, 1.0),
                    clamp(c, 0.0, 0.6),
                );
        }
    }
}

#[derive(Debug)]
struct Model {
    _window: WindowId,
    flow_field: FlowField,
    perlin: Perlin,
    zoff: f64,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let zoff = 0.0;
    let perlin = Perlin::new();
    let flow_field = FlowField::new(800, 800, |x, y| {
        (perlin.get([x * 0.01, y * 0.01, zoff]) + 1.0) / 2.0
    });

    Model {
        _window,
        flow_field,
        perlin,
        zoff,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let perlin = &model.perlin;
    let zoff = &model.zoff;
    model
        .flow_field
        .update(|x, y| (1.0 + perlin.get([x * 0.01, y * 0.01, *zoff])) / 2.0);

    model.zoff += 0.01;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().rgba(0.0, 0.0, 0.0, 0.01);

    model.flow_field.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
