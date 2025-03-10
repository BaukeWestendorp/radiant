use gpui::*;

pub struct Draggable {
    id: ElementId,

    position: Point<Pixels>,

    prev_mouse_pos: Option<Point<Pixels>>,

    child: AnyView,
}

impl Draggable {
    pub fn new(
        id: impl Into<ElementId>,
        position: Point<Pixels>,
        child: impl Into<AnyView>,
    ) -> Self {
        Self { id: id.into(), position, prev_mouse_pos: None, child: child.into() }
    }

    pub fn position(&self) -> &Point<Pixels> {
        &self.position
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
        let id = event.drag(cx);
        if &self.id != id {
            return;
        }

        let mouse_position = window.mouse_position();

        let diff = self.prev_mouse_pos.map_or(Point::default(), |prev| mouse_position - prev);

        self.position += diff;

        self.prev_mouse_pos = Some(mouse_position);
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.prev_mouse_pos = None;
        cx.emit(DraggableEvent::PositionCommitted(self.position));
    }
}

impl Render for Draggable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink()
            .absolute()
            .left(self.position.x)
            .top(self.position.y)
            .id(self.id.clone())
            .child(self.child.clone())
            .on_drag(self.id.clone(), |_, _, _, cx| cx.new(|_cx| EmptyView))
            .on_drag_move(cx.listener(Self::handle_drag_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
    }
}

impl EventEmitter<DraggableEvent> for Draggable {}

pub enum DraggableEvent {
    PositionCommitted(Point<Pixels>),
}
