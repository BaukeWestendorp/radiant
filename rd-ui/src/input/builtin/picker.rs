use gpui::{
    App, AppContext, ElementId, Entity, FocusHandle, Focusable, MouseButton, RenderOnce, Window,
    deferred, div, prelude::*,
};
use std::fmt::Display;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::{
    ActiveTheme, Button, ButtonVariant, HslaExt, INPUT_HEIGHT, Icon, IconSize, IconVariant,
    InputDelegate, InputEvent, InputState, LayoutDirection, container, h_flex,
    interactive_container, util::FocusableExt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PickerKind {
    #[default]
    Inline,
    Dropdown,
}

pub struct LabelMissing;
pub struct LabelProvided;

pub struct Picker<V: Clone + PartialEq> {
    value: Entity<V>,
    options: Vec<V>,
    focus_handle: FocusHandle,
    picker_kind: PickerKind,
    is_opened: bool,
    label_fn: Arc<dyn Fn(&V) -> String + 'static>,
}

pub struct PickerBuilder<V, LabelState> {
    value: V,
    options: Vec<V>,
    picker_kind: PickerKind,
    label_fn: Option<Arc<dyn Fn(&V) -> String + 'static>>,
    _marker: PhantomData<LabelState>,
}

impl<V: Clone + PartialEq + 'static> Picker<V> {
    pub fn builder(
        value: V,
        options: impl IntoIterator<Item = V>,
    ) -> PickerBuilder<V, LabelMissing> {
        PickerBuilder {
            value,
            options: options.into_iter().collect(),
            picker_kind: PickerKind::default(),
            label_fn: None,
            _marker: PhantomData,
        }
    }

    pub fn value<'a>(&'a self, cx: &'a App) -> &'a V {
        self.value.read(cx)
    }

    pub fn options(&self) -> &[V] {
        &self.options
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

impl<V: Clone + PartialEq + 'static, L> PickerBuilder<V, L> {
    pub fn kind(mut self, kind: PickerKind) -> Self {
        self.picker_kind = kind;
        self
    }

    pub fn inline(mut self) -> Self {
        self.picker_kind = PickerKind::Inline;
        self
    }

    pub fn dropdown(mut self) -> Self {
        self.picker_kind = PickerKind::Dropdown;
        self
    }
}

impl<V: Clone + PartialEq + 'static> PickerBuilder<V, LabelMissing> {
    pub fn label_fn(
        self,
        label_fn: impl Fn(&V) -> String + 'static,
    ) -> PickerBuilder<V, LabelProvided> {
        PickerBuilder {
            value: self.value,
            options: self.options,
            picker_kind: self.picker_kind,
            label_fn: Some(Arc::new(label_fn)),
            _marker: PhantomData,
        }
    }
}

impl<V: Clone + PartialEq + Display + 'static> PickerBuilder<V, LabelMissing> {
    pub fn build(
        self,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<InputState<Picker<V>>>,
    ) -> Picker<V> {
        self.label_fn(|v| v.to_string()).build(focus_handle, window, cx)
    }
}

impl<V: Clone + PartialEq + 'static> PickerBuilder<V, LabelProvided> {
    pub fn build(
        self,
        focus_handle: FocusHandle,
        _window: &mut Window,
        cx: &mut Context<InputState<Picker<V>>>,
    ) -> Picker<V> {
        let value = cx.new(|_| self.value);
        Picker {
            value,
            options: self.options,
            focus_handle,
            picker_kind: self.picker_kind,
            is_opened: false,
            label_fn: self.label_fn.expect("LabelProvided bound should mean this is valid"),
        }
    }
}

impl<V: Clone + PartialEq + 'static> InputDelegate for Picker<V> {
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

impl<V: Clone + PartialEq + 'static> Focusable for Picker<V> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(IntoElement)]
struct PickerElement<V: Clone + PartialEq + 'static> {
    state: Entity<InputState<Picker<V>>>,
}

impl<V: Clone + PartialEq + 'static> RenderOnce for PickerElement<V> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let picker_kind = self.state.read(cx).picker_kind;

        match picker_kind {
            PickerKind::Dropdown => self.render_dropdown(window, cx).into_any_element(),
            PickerKind::Inline => self.render_inline(window, cx).into_any_element(),
        }
    }
}

impl<V: Clone + PartialEq + 'static> PickerElement<V> {
    fn render_dropdown(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = ElementId::View(self.state.entity_id());
        let focus_handle = self.state.focus_handle(cx).clone();

        let (open, variants, label_fn) = {
            let state = self.state.read(cx);
            (state.is_opened, state.options.clone(), state.label_fn.clone())
        };

        let current_value_label = {
            let state = self.state.read(cx);
            let v = state.value.read(cx);
            (state.label_fn)(v)
        };

        let preview = h_flex()
            .size_full()
            .overflow_x_hidden()
            .whitespace_nowrap()
            .text_ellipsis() // FIXME: Why the fuck does this never properly work.
            .child(current_value_label);

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
                            let label = label_fn(&variant);
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

        let (variants, label_fn) = {
            let state = self.state.read(cx);
            (state.options.clone(), state.label_fn.clone())
        };

        div()
            .id(id)
            .track_focus(&focus_handle)
            .focus_ring(focus_handle.is_focused(window), window, cx)
            .w_full()
            .child(container(window, cx).w_full().p_1().flex().gap_1().children(
                variants.into_iter().enumerate().map(|(ix, variant)| {
                    let label = label_fn(&variant);
                    let selected = self.state.read(cx).value(cx) == &variant;

                    Button::new(("picker-value", ix), cx.focus_handle())
                        .label(label)
                        .variant(if selected {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Secondary
                        })
                        .w_full()
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
