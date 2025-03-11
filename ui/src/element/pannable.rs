use gpui::*;

use crate::utils::z_stack;

pub struct Pannable {
    id: ElementId,

    offset: Point<Pixels>,

    prev_mouse_pos: Option<Point<Pixels>>,

    child: AnyView,
}

impl Pannable {
    pub fn new(id: impl Into<ElementId>, offset: Point<Pixels>, child: impl Into<AnyView>) -> Self {
        Self { id: id.into(), offset, prev_mouse_pos: None, child: child.into() }
    }

    pub fn offset(&self) -> &Point<Pixels> {
        &self.offset
    }

    pub fn set_offset(&mut self, offset: Point<Pixels>, cx: &mut Context<Self>) {
        self.offset = offset;
        cx.emit(PannableEvent::OffsetCommitted(self.offset));
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
        self.offset += diff;

        self.prev_mouse_pos = Some(mouse_position);

        cx.emit(PannableEvent::OffsetChanged(self.offset));
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.prev_mouse_pos = None;
        cx.emit(PannableEvent::OffsetCommitted(self.offset));
    }
}

impl Render for Pannable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let hitbox = div()
            .id(self.id.clone())
            .on_drag(self.id.clone(), |_, _, _, cx| cx.new(|_cx| EmptyView))
            .on_drag_move(cx.listener(Self::handle_drag_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .size_full();

        let child =
            div().absolute().left(self.offset.x).top(self.offset.y).child(self.child.clone());

        z_stack([hitbox.into_any_element(), child.into_any_element()]).size_full()
    }
}

impl EventEmitter<PannableEvent> for Pannable {}

pub enum PannableEvent {
    OffsetChanged(Point<Pixels>),
    OffsetCommitted(Point<Pixels>),
}
