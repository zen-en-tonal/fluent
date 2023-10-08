use std::{ops::Range, vec::IntoIter};

use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(init).simple_window(view).size(600, 615).run()
}

struct State {
    perlin: Perlin,
}

fn init(_app: &App) -> State {
    State {
        perlin: Perlin::new(),
    }
}

fn view(app: &App, state: &State, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw();
    let rect = app.window_rect();
    for (x, y) in rect.xy_iter().step_by(50) {
        let v = state
            .perlin
            .get([x as f64 / 100.0, y as f64 / 100.0, app.time as f64 / 10.0])
            * TAU_F64;
        let arrow = Arrow::new(Point2::new(x as f32, y as f32), 10.0, v as f32);
        arrow.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap()
}

trait Bound {
    fn x_range(&self) -> Range<i32>;

    fn y_range(&self) -> Range<i32>;

    fn xy_iter(&self) -> IntoIter<(i32, i32)> {
        let mut vec = vec![];
        for x in self.x_range() {
            for y in self.y_range().clone() {
                vec.push((x, y));
            }
        }
        vec.into_iter()
    }
}

impl Bound for Rect {
    fn x_range(&self) -> Range<i32> {
        self.x.start as i32..self.x.end as i32
    }

    fn y_range(&self) -> Range<i32> {
        self.y.start as i32..self.y.end as i32
    }
}

struct Arrow {
    point: Point2,
    angle: f32,
    length: f32,
}

impl Arrow {
    fn new(point: Point2, length: f32, angle_rad: f32) -> Arrow {
        Arrow {
            point,
            length,
            angle: angle_rad,
        }
    }

    fn draw(&self, draw: &Draw) {
        let (start, end) = self.start_end();
        draw.arrow().start(start).end(end).weight(1.0).color(BLACK);
    }

    fn start_end(&self) -> (Vec2, Vec2) {
        let start = self.point.to_owned();
        let end = [
            self.length * self.angle.cos() + start.x,
            self.length * self.angle.sin() + start.y,
        ]
        .into();
        (start, end)
    }

    // fn centered_start_end(&self) -> (Vec2, Vec2) {
    //     let x = self.length / 2.0 * self.angle.cos();
    //     let y = self.length / 2.0 * self.angle.sin();
    //     let start = [self.point.x - x, self.point.y - y].into();
    //     let end = [self.point.x + x, self.point.y + y].into();
    //     (start, end)
    // }
}
