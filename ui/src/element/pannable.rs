use gpui::*;

use crate::utils::z_stack;

pub struct Pannable {
    id: ElementId,
    offset: Point<Pixels>,
    child: AnyView,
    mouse_button: MouseButton,

    prev_mouse_pos: Option<Point<Pixels>>,
    dragging: bool,
}

impl Pannable {
    pub fn new(id: impl Into<ElementId>, offset: Point<Pixels>, child: impl Into<AnyView>) -> Self {
        Self {
            id: id.into(),
            offset,
            child: child.into(),
            mouse_button: MouseButton::Left,

            prev_mouse_pos: None,
            dragging: false,
        }
    }

    pub fn mouse_button(&self) -> &MouseButton {
        &self.mouse_button
    }

    pub fn set_mouse_button(&mut self, mouse_button: MouseButton) {
        self.mouse_button = mouse_button;
    }

    pub fn offset(&self) -> &Point<Pixels> {
        &self.offset
    }

    pub fn set_offset(&mut self, offset: Point<Pixels>, cx: &mut Context<Self>) {
        self.offset = offset;
        cx.emit(PannableEvent::OffsetCommitted(self.offset));
    }

    pub fn dragging(&self) -> bool {
        self.dragging
    }
}

impl Pannable {
    fn handle_mouse_down(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.dragging = true;
    }

    fn handle_mouse_move(
        &mut self,
        _event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.dragging {
            return;
        }

        let mouse_position = window.mouse_position();

        let diff = self.prev_mouse_pos.map_or(Point::default(), |prev| mouse_position - prev);
        self.offset += diff;

        self.prev_mouse_pos = Some(mouse_position);

        cx.emit(PannableEvent::OffsetChanged(self.offset));
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.prev_mouse_pos = None;
        self.dragging = false;
        cx.emit(PannableEvent::OffsetCommitted(self.offset));
    }

    fn handle_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dragging {
            return;
        }

        let pixel_delta = event.delta.pixel_delta(window.line_height());

        self.offset += pixel_delta;

        if pixel_delta.is_zero() {
            cx.emit(PannableEvent::OffsetCommitted(self.offset));
        } else {
            cx.emit(PannableEvent::OffsetChanged(self.offset));
            cx.notify();
        }
    }
}

impl Render for Pannable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let hitbox = div()
            .id(self.id.clone())
            .on_mouse_down(self.mouse_button, cx.listener(Self::handle_mouse_down))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(self.mouse_button, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(self.mouse_button, cx.listener(Self::handle_mouse_up))
            .on_scroll_wheel(cx.listener(Self::handle_scroll_wheel))
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
