mod curves;

use druid::widget::prelude::*;
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, Color, Rect, LocalizedString, Point};
use druid::kurbo::BezPath;
use druid::widget::Label;

struct CustomWidget;

impl Widget<String> for CustomWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {}
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {}
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &String, data: &String, env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &String, env: &Env) -> Size {
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        // clear widget with one color
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        // draw from module
        curves::trig::draw_on(ctx);
    }
}

fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}

fn main() {
    //curves::trig::start_here();

    let window = WindowDesc::new(CustomWidget {});
    AppLauncher::with_window(window).launch("Druid Example".to_string()).expect("launch failed");

}
