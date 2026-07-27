use gpui::{
    App, AppContext, ElementId, Entity, FocusHandle, Focusable, MouseButton, RenderOnce, Window,
    deferred, div, prelude::*, px,
};

use crate::{
    ActiveTheme, Button, HslaExt, INPUT_HEIGHT, Icon, IconSize, IconVariant, InputDelegate,
    InputEvent, InputState, container, h_flex, interactive_container, styled_ext::FocusableExt,
};

use super::LayoutDirection;

pub trait PickerValue: Clone + PartialEq {
    fn variants() -> Vec<Self>
    where
        Self: Sized;

    fn label(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PickerKind {
    #[default]
    Inline,
    Dropdown,
}

pub struct Picker<V: PickerValue> {
    value: Entity<V>,
    focus_handle: FocusHandle,
    picker_kind: PickerKind,

    is_opened: bool,
}

impl<V: PickerValue + 'static> Picker<V> {
    pub fn new(
        value: V,
        focus_handle: FocusHandle,
        picker_kind: PickerKind,
        _window: &mut Window,
        cx: &mut Context<InputState<Self>>,
    ) -> Self {
        let value = cx.new(|_| value);
        Self { value, focus_handle, picker_kind, is_opened: false }
    }

    pub fn inline(
        value: V,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<InputState<Self>>,
    ) -> Self {
        Self::new(value, focus_handle, PickerKind::Inline, window, cx)
    }

    pub fn dropdown(
        value: V,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<InputState<Self>>,
    ) -> Self {
        Self::new(value, focus_handle, PickerKind::Dropdown, window, cx)
    }

    pub fn value<'a>(&'a self, cx: &'a App) -> &'a V {
        self.value.read(cx)
    }

    pub fn set_value(&mut self, value: V, cx: &mut Context<InputState<Self>>) {
        cx.emit(InputEvent::Submit(value.clone()));

        self.value.update(cx, |v, cx| {
            *v = value;
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

impl<V: PickerValue + 'static> InputDelegate for Picker<V> {
    type Value = V;

    fn new_element(
        state: Entity<InputState<Self>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        PickerElement { state }
    }

    fn value_or_default(&self, cx: &App) -> Self::Value {
        self.value.read(cx).clone()
    }

    fn form_layout_direction(&self) -> Option<LayoutDirection> {
        match self.picker_kind {
            PickerKind::Inline => None,
            PickerKind::Dropdown => Some(LayoutDirection::Horizontal),
        }
    }
}

impl<V: PickerValue + 'static> Focusable for Picker<V> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(IntoElement)]
struct PickerElement<V: PickerValue + 'static> {
    state: Entity<InputState<Picker<V>>>,
}

impl<V: PickerValue + 'static> RenderOnce for PickerElement<V> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let picker_kind = self.state.read(cx).picker_kind;

        match picker_kind {
            PickerKind::Dropdown => self.render_dropdown(window, cx).into_any_element(),
            PickerKind::Inline => self.render_inline(window, cx).into_any_element(),
        }
    }
}

impl<V: PickerValue + 'static> PickerElement<V> {
    fn render_dropdown(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = ElementId::View(self.state.entity_id());
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
            let picker = deferred(
                container(window, cx)
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
                    .when(cx.theme().shadow, |e| e.shadow_md()),
            );

            Some(picker)
        } else {
            None
        };

        div()
            .relative()
            .child(
                interactive_container(id, Some(focus_handle))
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

    fn render_inline(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = ElementId::View(self.state.entity_id());
        let focus_handle = self.state.focus_handle(cx);
        let variants = V::variants();

        div()
            .id(id)
            .track_focus(&focus_handle)
            .focus_ring(focus_handle.is_focused(window), px(1.0), window, cx)
            .child(container(window, cx).p_1().flex().gap_1().children(
                variants.into_iter().enumerate().map(|(ix, variant)| {
                    let label = variant.label();

                    let selected = self.state.read(cx).value(cx) == &variant;

                    Button::new(("picker-value", ix))
                        .selected(selected)
                        .focusable(false)
                        .w_full()
                        .child(label)
                        .block_mouse_except_scroll()
                        .on_click({
                            let state = self.state.clone();
                            let variant = variant.clone();
                            move |_, _, cx| {
                                state.update(cx, |state, cx| {
                                    state.set_value(variant.clone(), cx);
                                    cx.notify();
                                });
                            }
                        })
                }),
            ))
    }
}
