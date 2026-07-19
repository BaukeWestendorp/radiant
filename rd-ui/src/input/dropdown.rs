use gpui::{
    App, ElementId, Entity, FocusHandle, IntoElement, MouseButton, RenderOnce, Styled, Window, div,
    prelude::*,
};

use crate::{
    ActiveTheme, HslaExt, INPUT_HEIGHT, Icon, IconSize, IconVariant, container, h_flex,
    interactive_container,
};

pub trait DropdownItem: Clone + 'static {
    fn label(&self) -> String;

    fn variants() -> Vec<Self>
    where
        Self: Sized;
}

pub struct DropdownState<T>
where
    T: DropdownItem,
{
    value: Entity<Option<T>>,
    focus_handle: FocusHandle,
    open: bool,
}

impl<T> DropdownState<T>
where
    T: DropdownItem,
{
    pub fn new(value: Entity<Option<T>>, cx: &mut Context<Self>) -> Self {
        Self { value, focus_handle: cx.focus_handle(), open: false }
    }

    pub fn open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn set_value(&mut self, new_value: T, cx: &mut App) {
        self.value.update(cx, |value, cx| {
            *value = Some(new_value);
            cx.notify();
        });

        self.open(false);
    }
}

#[derive(IntoElement)]
pub struct Dropdown<T>
where
    T: DropdownItem,
{
    id: ElementId,
    state: Entity<DropdownState<T>>,
}

impl<T> Dropdown<T>
where
    T: DropdownItem,
{
    pub fn new(id: impl Into<ElementId>, state: Entity<DropdownState<T>>) -> Self {
        Self { id: id.into(), state }
    }

    fn value<'a>(&self, cx: &'a App) -> Option<&'a T> {
        self.state.read(cx).value.read(cx).as_ref()
    }

    fn value_label(&self, cx: &App) -> Option<String> {
        self.value(cx).map(|v| v.label())
    }

    fn variants(&self) -> Vec<T> {
        T::variants()
    }
}

impl<T> RenderOnce for Dropdown<T>
where
    T: DropdownItem,
{
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self.state.read(cx).focus_handle.clone();
        let open = self.state.read(cx).open;

        let preview = match self.value_label(cx) {
            Some(label) => h_flex()
                .size_full()
                .overflow_x_hidden()
                .whitespace_nowrap()
                .text_ellipsis() // FIXME: Why the fuck does this never properly work.
                .child(label),
            None => h_flex().size_full().text_color(cx.theme().fg_secondary).child("..."),
        };

        let icon = Icon::new(IconVariant::ChevronDown, IconSize::Small);

        let picker = open.then({
            let variants = self.variants();
            let state = self.state.clone();

            move || {
                container(window, cx)
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
                                    let state = state.clone();
                                    let variant_clone = variant.clone();
                                    move |_, _, cx| {
                                        state.update(cx, |state, cx| {
                                            state.set_value(variant_clone.clone(), cx);
                                            cx.notify();
                                        })
                                    }
                                })
                                .on_mouse_up(MouseButton::Left, {
                                    let state = state.clone();
                                    let variant_clone = variant.clone();
                                    move |_, _, cx| {
                                        state.update(cx, |state, cx| {
                                            state.set_value(variant_clone.clone(), cx);
                                            cx.notify();
                                        })
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
                    .when(cx.theme().shadow, |e| e.shadow_md())
            }
        });

        div()
            .relative()
            .child(
                interactive_container(self.id, Some(focus_handle))
                    .px_2()
                    .w_full()
                    .h(INPUT_HEIGHT)
                    .on_mouse_down(MouseButton::Left, {
                        let state = self.state.clone();
                        move |_, _, cx| {
                            state.update(cx, |state, cx| {
                                state.toggle();
                                cx.notify();
                            })
                        }
                    })
                    .on_mouse_up_out(MouseButton::Left, {
                        let state = self.state.clone();
                        move |_, _, cx| {
                            state.update(cx, |state, cx| {
                                state.open(false);
                                cx.notify();
                            })
                        }
                    })
                    .on_mouse_down_out({
                        let state = self.state.clone();
                        move |_, _, cx| {
                            state.update(cx, |state, cx| {
                                state.open(false);
                                cx.notify();
                            })
                        }
                    })
                    .child(h_flex().size_full().gap_2().child(preview).child(icon)),
            )
            .children(picker)
    }
}
