use crate::utils::snap_point;
use gpui::*;

pub struct Draggable {
    id: ElementId,

    position: Point<Pixels>,
    snap_threshold: Option<Pixels>,

    prev_mouse_pos: Option<Point<Pixels>>,

    child: AnyView,
}

impl Draggable {
    pub fn new(
        id: impl Into<ElementId>,
        position: Point<Pixels>,
        snap_threshold: Option<Pixels>,
        child: impl Into<AnyView>,
    ) -> Self {
        Self { id: id.into(), position, prev_mouse_pos: None, snap_threshold, child: child.into() }
    }

    pub fn position(&self) -> &Point<Pixels> {
        &self.position
    }

    pub fn snap_threshold(&self) -> Option<Pixels> {
        self.snap_threshold
    }

    pub fn snapped_position(&self) -> Point<Pixels> {
        match self.snap_threshold {
            Some(threshold) => snap_point(self.position, threshold),
            None => self.position,
        }
    }

    pub fn set_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        self.position = position;
        cx.emit(DraggableEvent::PositionCommitted(self.position));
    }

    fn handle_drag_move(
        &mut self,
        event: &DragMoveEvent<ElementId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if &self.id != event.drag(cx) {
            return;
        }

        let mouse_position = window.mouse_position();

        let diff = self.prev_mouse_pos.map_or(Point::default(), |prev| mouse_position - prev);
        self.position += diff;

        self.prev_mouse_pos = Some(mouse_position);

        cx.emit(DraggableEvent::PositionChanged(self.position));
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.prev_mouse_pos = None;
        cx.emit(DraggableEvent::PositionCommitted(self.position));
    }
}

impl Render for Draggable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let position = self.snapped_position();

        div()
            .id(self.id.clone())
            .flex()
            .flex_shrink()
            .absolute()
            .left(position.x)
            .top(position.y)
            .child(self.child.clone())
            .on_drag(self.id.clone(), |_, _, _, cx| cx.new(|_cx| EmptyView))
            .on_drag_move(cx.listener(Self::handle_drag_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
    }
}

impl EventEmitter<DraggableEvent> for Draggable {}

pub enum DraggableEvent {
    PositionChanged(Point<Pixels>),
    PositionCommitted(Point<Pixels>),
}
