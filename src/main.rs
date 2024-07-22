use gpui::*;
use line_segments::LineSegments;

mod line_segments;

pub struct MainView {}

impl MainView {
    fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }
}

impl Render for MainView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let path = (0..20)
            .map(|idx| point(idx as f32 / 20.0, f32::sin(idx as f32 / 20.0 * 6.0) * 0.5))
            .collect();

        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(white())
            .child(LineSegments {
                size: size(600.0.into(), 300.0.into()),
                path_width: px(2.0),
                path_color: blue(),
                points: path,
            })
    }
}

fn main() {
    App::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| MainView::new(cx))
            .unwrap();
    })
}
