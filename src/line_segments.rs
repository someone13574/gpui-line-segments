use gpui::*;

pub struct LineSegments {
    pub size: Size<Pixels>,
    pub path_color: Hsla,
    pub path_width: Pixels,
    pub points: Vec<Point<f32>>,
}

impl IntoElement for LineSegments {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct LineSegmentsPrepaintState {
    path: Option<Path<Pixels>>,
}

impl Element for LineSegments {
    type RequestLayoutState = ();

    type PrepaintState = LineSegmentsPrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = self.size.width.into();
        style.size.height = self.size.height.into();

        (cx.request_layout(style, []), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        let points = self
            .points
            .iter()
            .map(|point| point_to_px(point, bounds))
            .collect::<Vec<_>>();

        let mut path = None;
        let mut return_points = Vec::new();
        let mut prev_point = None;

        for (idx, point) in points.clone().into_iter().enumerate() {
            let (return_point, path_point) = offset_points(
                prev_point,
                point,
                points.get(idx + 1).cloned(),
                (f32::from(self.path_width) / 2.0).into(),
            );

            add_point(&mut path, path_point);
            return_points.push(return_point);
            prev_point = Some(point);
        }
        for return_point in return_points.into_iter().rev() {
            add_point(&mut path, return_point);
        }

        Self::PrepaintState { path }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        if let Some(path) = prepaint.path.take() {
            cx.paint_path(path, self.path_color);
        }
    }
}

fn point_to_px(point: &Point<f32>, bounds: Bounds<Pixels>) -> Point<Pixels> {
    Point {
        x: point.x * bounds.size.width + bounds.origin.x,
        y: point.y * bounds.size.height + bounds.origin.y,
    }
}

fn offset_points(
    prev: Option<Point<Pixels>>,
    curr: Point<Pixels>,
    next: Option<Point<Pixels>>,
    offset: f32,
) -> (Point<Pixels>, Point<Pixels>) {
    let mut bisector_angle = 0.0;
    let mut count = 0;

    if let Some(point) = prev {
        bisector_angle -= f32::from(curr.y - point.y).atan2((curr.x - point.x).into());
        count += 1;
    }
    if let Some(point) = next {
        bisector_angle -= f32::from(point.y - curr.y).atan2((point.x - curr.x).into());
        count += 1;
    }

    bisector_angle /= count as f32;

    let offset_x = (offset * bisector_angle.sin()).into();
    let offset_y = (offset * bisector_angle.cos()).into();

    let p1 = Point::new(curr.x + offset_x, curr.y + offset_y);
    let p2 = Point::new(curr.x - offset_x, curr.y - offset_y);

    (p1, p2)
}

fn add_point(path: &mut Option<Path<Pixels>>, point: Point<Pixels>) {
    if let Some(path) = path {
        path.line_to(point);
    } else {
        *path = Some(Path::new(point));
    }
}
