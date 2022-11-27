use std::f64::consts::PI;
use druid::kurbo::BezPath;
use druid::{Color, PaintCtx, Point, RenderContext};
use druid::kurbo::Line;

pub fn start_here() {
    for i in (0..63).map(|x| x as f64 * 0.1) {
        println!("{}", f64::sin(i));
    }
}

pub fn draw_on(ctx: &mut PaintCtx) {
    let size = ctx.size();
    let stroke_width = 2.5;
    let stroke_color = Color::rgb8(0, 128, 0);

    // Create an arbitrary bezier path
    // let mut path = BezPath::new();
    // path.move_to(Point::ORIGIN);
    // path.quad_to((40.0, 50.0), (size.width, size.height));
    // ctx.stroke(path, &stroke_color, stroke_width);

    // paint
    let amplitude = size.height * 0.45;
    let freq = 5.6;

    let mut prev_x = 0.;
    let mut prev_y = size.height / 2.0;
    for i in 0..size.width as i32 {
        let x = i as f64;
        let y = size.height / 2. + f64::sin(x / size.width * PI * 2.0 * freq) * amplitude;

        let p0 = Point::new(prev_x, prev_y);
        let p1 = Point::new(x, y);
        let line = Line::new(p0, p1);
        ctx.stroke(line, &stroke_color, stroke_width);

        (prev_x, prev_y) = (x, y);
    }
}