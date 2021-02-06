use nannou::prelude::*;

const GAP: i32 = 10;

#[derive(Debug)]
struct FlowPoint {
    pos: Vector2,
}

impl FlowPoint {
    fn new(x: i32, y: i32) -> FlowPoint {
        FlowPoint {
            pos: vec2(x as f32, y as f32),
        }
    }
}

#[derive(Debug)]
struct FlowField {
    points: Vec<FlowPoint>,
}

impl FlowField {
    fn new(width: i32, height: i32) -> FlowField {
        let mut points = Vec::new();

        let max_x = ((width / 2) / GAP) + GAP;
        let min_x = -(width / 2) / GAP;
        let min_y = -(height / 2) / GAP;
        let max_y = ((height / 2) / GAP) + GAP;

        for x in min_x..max_x {
            for y in min_y..max_y {
                let fp = FlowPoint::new(x * GAP, y * GAP);

                println!("x: {}, y: {}", x, y);
                points.push(fp);
            }
        }

        FlowField { points }
    }

    fn draw(&self, draw: &Draw) {
        for point in &self.points {
            draw.ellipse().x(point.pos.x).y(point.pos.y).radius(2.0).color(WHITE);
        }
    }
}

struct Model {
    _window: WindowId,
    flow_field: FlowField,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let flow_field = FlowField::new(800, 800);

    Model {
        _window,
        flow_field,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.flow_field.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
