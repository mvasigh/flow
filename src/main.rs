use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

const GAP: i32 = 10;

#[derive(Debug)]
struct FlowPoint {
    pos: Vector2,
    mag: f64,
}

impl FlowPoint {
    fn new(x: i32, y: i32, mag: f64) -> FlowPoint {
        FlowPoint {
            pos: vec2(x as f32, y as f32),
            mag,
        }
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
                let px = x * GAP;
                let py = y * GAP;
                let fp = FlowPoint::new(px, py, magnitude(px as f64, py as f64));
                points.push(fp);
            }
        }

        FlowField { points }
    }

    fn draw(&self, draw: &Draw) {
        for point in &self.points {
            draw.ellipse()
                .x(point.pos.x)
                .y(point.pos.y)
                .radius((point.mag as f32) * 2.0)
                .color(WHITE);
        }
    }
}

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
    let perlin = Perlin::default();
    let flow_field = FlowField::new(800, 800, |x, y| perlin.get([x * 0.01, y * 0.01, zoff]));

    Model {
        _window,
        flow_field,
        perlin,
        zoff,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.zoff += 0.01;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.flow_field.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
