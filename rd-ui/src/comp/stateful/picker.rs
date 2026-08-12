use gpui::{
    App, ElementId, EventEmitter, FocusHandle, Focusable, MouseButton, MouseMoveEvent,
    SharedString, Window, deferred, div, prelude::*,
};

use crate::{
    ActiveTheme, Emphasis, HslaExt, StyledExt, StyledParentExt,
    StyledStatefulInteractiveElementExt,
    comp::{
        Button, ButtonVariant, Disableable, FocusableComponent, Icon, IconSize, IconVariant,
        Identifiable, Labelled,
        stateful::{self, FormWidget},
    },
};

pub(crate) mod action {
    pub const KEY_CONTEXT: &str = "Picker";

    gpui::actions!(picker, [Next, Previous, Clear]);
}

pub struct Picker<T> {
    id: ElementId,
    focus_handle: FocusHandle,

    items: Vec<PickerItem<T>>,
    selection: Option<usize>,
    open: bool,
    kind: PickerKind,
    disabled: bool,
}

impl<T: Clone + 'static> Picker<T> {
    pub fn new(
        id: impl Into<ElementId>,
        items: Vec<PickerItem<T>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let items = items
            .into_iter()
            .map(|item| item.with_focus_handle(cx.focus_handle().tab_stop(true), cx))
            .collect();

        Self {
            id: id.into(),
            focus_handle: cx.focus_handle().tab_stop(true),

            items,
            selection: None,
            open: false,
            kind: PickerKind::default(),
            disabled: false,
        }
    }

    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    pub fn with_selection(mut self, selection: Option<usize>) -> Self {
        self.selection = selection;
        self
    }

    pub fn set_selection(&mut self, selection: Option<usize>, cx: &mut Context<Self>) {
        self.selection = selection;
        let value = self.selected_item().as_ref().map(|item| item.value.clone());
        cx.emit(stateful::event::Submit(value.clone()));
        cx.emit(stateful::event::Change(value));
        cx.notify();
    }

    pub fn items(&self) -> &[PickerItem<T>] {
        &self.items
    }

    pub fn selected_item(&self) -> Option<&PickerItem<T>> {
        self.selection.and_then(|index| self.items.get(index))
    }

    pub fn kind(&self) -> PickerKind {
        self.kind
    }

    pub fn with_kind(mut self, kind: PickerKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn set_kind(&mut self, kind: PickerKind, cx: &mut Context<Self>) {
        self.kind = kind;
        cx.notify()
    }

    pub fn open(&self) -> bool {
        self.open
    }

    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.open = open;

        if self.open {
            if let Some(selection) = self.selection {
                if let Some(focus_handle) = &self.items[selection].focus_handle {
                    focus_handle.focus(window, cx);
                }
            } else {
                self.set_selection(Some(0), cx);
                if let Some(first_item) = self.items.first() {
                    if let Some(focus_handle) = &first_item.focus_handle {
                        focus_handle.focus(window, cx);
                    }
                }
            }
        }

        cx.notify();
    }

    pub fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_open(!self.open, window, cx);
    }

    fn handle_next(&mut self, _: &action::Next, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selection) = self.selection {
            let next = (selection + 1).min(self.items.len() - 1);
            self.set_selection(Some(next), cx);
            if let Some(focus_handle) = &self.items[next].focus_handle {
                focus_handle.focus(window, cx);
            }
        }
    }

    fn handle_previous(
        &mut self,
        _: &action::Previous,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selection) = self.selection {
            let prev = selection.saturating_sub(1);
            self.set_selection(Some(prev), cx);
            if let Some(focus_handle) = &self.items[prev].focus_handle {
                focus_handle.focus(window, cx);
            }
        }
    }

    fn handle_clear(&mut self, _: &action::Clear, window: &mut Window, cx: &mut Context<Self>) {
        self.set_selection(None, cx);
        self.set_open(false, window, cx);
        self.focus_handle.focus(window, cx);
    }

    fn render_dropdown(
        &mut self,
        searchable: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let selected_label = self
            .selected_item()
            .map(|item| item.label.clone())
            .unwrap_or_else(|| Some(SharedString::from("Select...")));
        let selected_icon = self.selected_item().and_then(|item| item.icon());

        let preview = div()
            .id(self.id.clone())
            .when(!self.disabled, |e| {
                e.track_focus(&self.focus_handle).focus_ring(&self.focus_handle, window, cx)
            })
            .h_flex()
            .justify_between()
            .gap_2()
            .size_full()
            .px_3()
            .when(self.disabled, |e| e.disabled_emphasis_bordered(Emphasis::Secondary, cx))
            .when(!self.disabled, |e| {
                e.interactive_emphasis_bordered(Emphasis::Secondary, cx)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, window, cx| {
                            this.set_open(true, window, cx);
                        }),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.set_open(true, window, cx);
                    }))
            })
            .when(self.open, |e| e.rounded_b_none())
            .child(
                div()
                    .h_flex()
                    .h_full()
                    .gap_2()
                    .when_some(selected_icon, |e, icon| {
                        e.child(Icon::new(icon, IconSize::ExtraSmall))
                    })
                    .text_color(cx.theme().fg_primary)
                    .when(self.selection().is_none(), |e| e.text_color(cx.theme().fg_secondary))
                    .child(selected_label.unwrap_or_default()),
            )
            .child(Icon::new(IconVariant::ChevronDown, IconSize::ExtraSmall));

        let picker = deferred(
            div()
                .absolute()
                .top_full()
                .w_full()
                .p_1()
                .v_flex()
                .when(self.disabled, |e| e.disabled_emphasis_bordered(Emphasis::Primary, cx))
                .when(!self.disabled, |e| e.emphasis_bordered(Emphasis::Primary, cx))
                .rounded_t_none()
                .border_t_0()
                .when(cx.theme().shadow, |e| e.shadow_md())
                .when(searchable, |e| {
                    e.child(
                        div()
                            .id("picker_search")
                            .block_mouse_except_scroll()
                            .interactive_emphasis_bordered(Emphasis::Secondary, cx)
                            .text_color(cx.theme().fg_tertiary)
                            .px_2()
                            .mb_1()
                            .child("Search..."),
                    )
                })
                .block_mouse_except_scroll()
                .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                    this.set_open(false, window, cx);
                }))
                .on_action::<action::Next>(cx.listener(Self::handle_next))
                .on_action::<action::Previous>(cx.listener(Self::handle_previous))
                .children(self.items.iter().enumerate().map(|(ix, item)| {
                    let selected = self.selection == Some(ix);
                    let icon = item.icon();
                    let label = item.label().to_owned();

                    let focus_handle = item.focus_handle.clone().expect("picker item focus handle");

                    div()
                        .id(("picker_value", ix))
                        .track_focus(&focus_handle)
                        .focus_ring(&focus_handle, window, cx)
                        .w_full()
                        .px_2()
                        .h_flex()
                        .gap_2()
                        .when(self.disabled, |e| {
                            if selected {
                                e.disabled_emphasis_bordered(Emphasis::Selected, cx)
                                    .text_color(cx.theme().fg_primary.disabled())
                            } else {
                                e.disabled_emphasis_bordered(Emphasis::Ghost, cx)
                                    .bg(Emphasis::Ghost.bg_color(cx).disabled())
                                    .border_color(Emphasis::Ghost.border_color(cx).disabled())
                            }
                        })
                        .when(!self.disabled, |e| {
                            let e = if selected {
                                e.interactive_emphasis_bordered(Emphasis::Selected, cx)
                                    .text_color(cx.theme().fg_primary)
                            } else {
                                e.emphasis_bordered(Emphasis::Ghost, cx)
                                    .hover(|e| e.bg(cx.theme().bg_secondary))
                                    .active(|e| e.bg(cx.theme().bg_tertiary))
                            };

                            e.on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, cx| {
                                    this.focus_handle.focus(window, cx);
                                    cx.notify();
                                }),
                            )
                            .on_mouse_move(cx.listener(
                                move |this, event: &MouseMoveEvent, _, cx| {
                                    if event.dragging() {
                                        this.set_selection(Some(ix), cx);
                                        cx.notify();
                                    }
                                },
                            ))
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, cx| {
                                    this.set_selection(Some(ix), cx);
                                    this.set_open(false, window, cx);
                                    this.focus_handle.focus(window, cx);
                                    cx.notify();
                                }),
                            )
                            .on_click(cx.listener(
                                move |this, _, window, cx| {
                                    this.set_selection(Some(ix), cx);
                                    this.set_open(false, window, cx);
                                    this.focus_handle.focus(window, cx);
                                    cx.notify();
                                },
                            ))
                        })
                        .when_some(icon, |e, icon| e.child(Icon::new(icon, IconSize::ExtraSmall)))
                        .when_some(label, |e, label| e.child(label.clone()))
                })),
        );

        div()
            .id(self.id.clone())
            .key_context(action::KEY_CONTEXT)
            .on_action::<action::Clear>(cx.listener(Self::handle_clear))
            .v_flex()
            .h(crate::comp::INPUT_SIZE)
            .gap_1()
            .block_mouse_except_scroll()
            .relative()
            .child(preview)
            .when(self.open, |e| e.child(picker))
    }

    fn render_inline(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id(self.id.clone())
            .key_context(action::KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action::<action::Next>(cx.listener(Self::handle_next))
            .on_action::<action::Previous>(cx.listener(Self::handle_previous))
            .on_action::<action::Clear>(cx.listener(Self::handle_clear))
            .p_0p5()
            .block_mouse_except_scroll()
            .emphasis_bordered(Emphasis::Secondary, cx)
            .child(div().h_flex().gap_1().children(self.items.iter().enumerate().map(
                |(ix, item)| {
                    let selected = self.selection == Some(ix);
                    let mut button = Button::new(ix, window, cx)
                        .with_focus_handle(
                            item.focus_handle.clone().expect("picker item focus handle"),
                            cx,
                        )
                        .with_disabled(self.disabled, cx)
                        .with_variant(if selected {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Secondary
                        })
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.set_selection(Some(ix), cx)),
                        );

                    button.set_label(item.label.clone());
                    button.set_icon(item.icon);
                    button.w_full()
                },
            )))
    }
}

impl<T> Disableable for Picker<T> {
    fn disabled(&self, _cx: &App) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool, _cx: &mut App) {
        self.disabled = disabled;
    }
}

impl<T> Identifiable for Picker<T> {
    fn id(&self, _cx: &App) -> &ElementId {
        &self.id
    }
}

impl<T: 'static> Focusable for Picker<T> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl<T: 'static> FocusableComponent for Picker<T> {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = focus_handle;
    }
}

impl<T: 'static> EventEmitter<stateful::event::Change<Option<T>>> for Picker<T> {}
impl<T: 'static> EventEmitter<stateful::event::Submit<Option<T>>> for Picker<T> {}

#[cfg(feature = "facet")]
impl<'facet, T: facet::Facet<'facet> + 'static> Picker<T> {
    pub fn from_facet(
        id: impl Into<ElementId>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<Self> {
        use facet_reflect::{Partial, peek_enum_variants};

        let variants = peek_enum_variants(T::SHAPE)
            .ok_or_else(|| anyhow::anyhow!("Picker can only be created from enum facets"))?;

        let items = variants
            .iter()
            .enumerate()
            .map(|(ix, variant)| {
                let label = variant.name;

                let value = Partial::alloc::<T>()
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to allocate enum variant for Picker: {e}")
                    })?
                    .select_nth_variant(ix)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to select enum variant at index {ix}: {e}")
                    })?
                    .build()
                    .map_err(|e| anyhow::anyhow!("Failed to build variant at index {ix}: {e}"))?
                    .materialize()
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to materialize variant at index {ix}: {e}")
                    })?;

                Ok(PickerItem::new(label, value)
                    .with_focus_handle(cx.focus_handle().tab_stop(true), cx))
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        Ok(Self {
            id: id.into(),
            items,
            selection: None,
            open: false,
            kind: PickerKind::default(),
            disabled: false,
            focus_handle: cx.focus_handle(),
        })
    }
}

impl<T: Clone + 'static> Render for Picker<T> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.kind {
            PickerKind::Auto => {
                let searchable = self.items.len() > 8;
                if self.items.len() > 4 {
                    self.render_dropdown(searchable, window, cx).into_any_element()
                } else {
                    if self
                        .items
                        .iter()
                        .map(|item| item.label().map(|l| l.len()).unwrap_or_default())
                        .sum::<usize>()
                        > 30
                    {
                        self.render_dropdown(searchable, window, cx).into_any_element()
                    } else {
                        self.render_inline(window, cx).into_any_element()
                    }
                }
            }
            PickerKind::Inline => self.render_inline(window, cx).into_any_element(),
            PickerKind::Dropdown { searchable } => {
                self.render_dropdown(searchable, window, cx).into_any_element()
            }
        }
    }
}

impl<T: Clone + PartialEq + 'static> FormWidget<Option<T>> for Picker<T> {
    fn get_value(&self, _cx: &App) -> Option<T> {
        Some(self.selected_item()?.value.clone())
    }

    fn set_value(&mut self, value: Option<T>, cx: &mut Context<Self>) {
        if let Some(value) = value {
            // FIXME: Add helper for this.
            if let Some(index) = self.items.iter().position(|item| item.value == value) {
                self.set_selection(Some(index), cx);
            } else {
                self.set_selection(None, cx);
            }
        } else {
            self.set_selection(None, cx);
        }
    }
}

pub struct PickerItem<T> {
    label: Option<SharedString>,
    icon: Option<IconVariant>,
    focus_handle: Option<FocusHandle>,
    value: T,
}

impl<T> PickerItem<T> {
    pub fn new(label: impl Into<SharedString>, value: T) -> Self {
        Self { label: Some(label.into()), icon: None, focus_handle: None, value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn with_value(mut self, value: T) -> Self {
        self.value = value;
        self
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

impl<T> Labelled for PickerItem<T> {
    fn label(&self) -> Option<&SharedString> {
        self.label.as_ref()
    }

    fn set_label(&mut self, label: impl Into<Option<SharedString>>) {
        self.label = label.into();
    }

    fn icon(&self) -> Option<IconVariant> {
        self.icon
    }

    fn set_icon(&mut self, icon: impl Into<Option<IconVariant>>) {
        self.icon = icon.into();
    }
}

impl<T: 'static> Focusable for PickerItem<T> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone().unwrap_or_else(|| cx.focus_handle())
    }
}

impl<T: 'static> FocusableComponent for PickerItem<T> {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = Some(focus_handle)
    }
}

impl<T: std::fmt::Display> From<T> for PickerItem<T> {
    fn from(value: T) -> Self {
        Self::new(value.to_string(), value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PickerKind {
    #[default]
    Auto,
    Inline,
    Dropdown {
        searchable: bool,
    },
}
