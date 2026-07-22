use gpui::{
    App, ElementId, Entity, FocusHandle, Focusable, MouseButton, RenderOnce, Window, div,
    prelude::*,
};

use crate::{
    ActiveTheme, HslaExt, INPUT_HEIGHT, Icon, IconSize, IconVariant, InputDelegate, InputEvent,
    InputState, container, h_flex, interactive_container,
};

pub trait DropdownValue: Clone {
    fn variants() -> Vec<Self>
    where
        Self: Sized;

    fn label(&self) -> String;
}

pub struct Dropdown<V: DropdownValue> {
    value: Entity<V>,

    focus_handle: FocusHandle,

    is_opened: bool,
}

impl<V: DropdownValue + 'static> Dropdown<V> {
    pub fn new(
        value: V,
        focus_handle: FocusHandle,
        _window: &mut Window,
        cx: &mut Context<InputState<Self>>,
    ) -> Self {
        let value = cx.new(|_| value);
        Self { value, focus_handle, is_opened: false }
    }

    pub fn value(&self) -> &Entity<V> {
        &self.value
    }

    pub fn set_value(&mut self, new_value: V, cx: &mut Context<InputState<Self>>) {
        cx.emit(InputEvent::Submit(new_value.clone()));

        self.value.update(cx, |value, cx| {
            *value = new_value;
            cx.notify();
        });

        self.open(false, cx);
    }

    pub fn is_opened(&self) -> bool {
        self.is_opened
    }

    pub fn toggle(&mut self, cx: &mut Context<InputState<Self>>) {
        self.is_opened = !self.is_opened;
        cx.notify();
    }

    pub fn open(&mut self, open: bool, cx: &mut Context<InputState<Self>>) {
        self.is_opened = open;
        cx.notify();
    }
}

impl<V: DropdownValue + 'static> InputDelegate for Dropdown<V> {
    type Value = V;

    fn new_element(
        state: Entity<InputState<Self>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        DropdownElement { id: ElementId::View(state.entity_id()), state }
    }
}

impl<V: DropdownValue + 'static> Focusable for Dropdown<V> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(IntoElement)]
struct DropdownElement<V: DropdownValue + 'static> {
    id: ElementId,
    state: Entity<InputState<Dropdown<V>>>,
}

impl<V: DropdownValue + 'static> RenderOnce for DropdownElement<V> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self.state.focus_handle(cx).clone();
        let open = self.state.read(cx).is_opened;
        let variants = V::variants();

        let preview = h_flex()
            .size_full()
            .overflow_x_hidden()
            .whitespace_nowrap()
            .text_ellipsis() // FIXME: Why the fuck does this never properly work.
            .child(self.state.read(cx).value.read(cx).label());

        let icon = Icon::new(IconVariant::ChevronDown, IconSize::Small);

        let picker = if open {
            let picker = container(window, cx)
                .occlude()
                .mt_1()
                .absolute()
                .top_full()
                .min_w_full()
                .child(div().flex().flex_col().gap_1().child(div().children(
                    variants.into_iter().enumerate().map(|(ix, variant)| {
                        let label = variant.label();
                        div()
                            .group("picker-list")
                            .px_1()
                            .py_0p5()
                            .when(ix != 0, |e| e.border_t_1())
                            .border_color(cx.theme().border_secondary)
                            .on_mouse_down(MouseButton::Left, {
                                let state = self.state.clone();
                                let variant = variant.clone();
                                move |_, _, cx| {
                                    state.update(cx, |state, cx| {
                                        state.set_value(variant.clone(), cx);
                                        cx.notify();
                                    });
                                }
                            })
                            .on_mouse_up(MouseButton::Left, {
                                let state = self.state.clone();
                                let variant = variant.clone();
                                move |_, _, cx| {
                                    state.update(cx, |state, cx| {
                                        state.set_value(variant.clone(), cx);
                                        cx.notify();
                                    });
                                }
                            })
                            .child(
                                div()
                                    .id(ix)
                                    .bg(cx.theme().bg_secondary)
                                    .border_1()
                                    .rounded(cx.theme().radius)
                                    .group_hover("picker-list", |e| {
                                        e.bg(cx.theme().bg_secondary.hover())
                                            .border_color(cx.theme().border_secondary)
                                    })
                                    .group_active("picker-list", |e| {
                                        e.bg(cx.theme().bg_secondary.active())
                                            .border_color(cx.theme().border_secondary)
                                    })
                                    .px_1()
                                    .whitespace_nowrap()
                                    .child(label),
                            )
                    }),
                )))
                .when(cx.theme().shadow, |e| e.shadow_md());

            Some(picker)
        } else {
            None
        };

        div()
            .relative()
            .child(
                interactive_container(self.id, Some(focus_handle))
                    .px_2()
                    .w_full()
                    .h(INPUT_HEIGHT)
                    .on_mouse_down(MouseButton::Left, {
                        let state = self.state.clone();
                        move |_, _, cx| state.update(cx, |state, cx| state.toggle(cx))
                    })
                    .on_mouse_down_out({
                        let state = self.state.clone();
                        move |_, _, cx| state.update(cx, |state, cx| state.open(false, cx))
                    })
                    .child(h_flex().size_full().gap_2().child(preview).child(icon)),
            )
            .children(picker)
    }
}
