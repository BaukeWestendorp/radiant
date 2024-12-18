use crate::{bounds_updater, ActiveTheme, ContainerKind, InteractiveContainer, StyledExt};
use gpui::*;
use prelude::FluentBuilder;

actions!(selector, [Enter, Escape, Up, Down]);

const KEY_CONTEXT: &str = "Selector";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("escape", Escape, Some(KEY_CONTEXT)),
        KeyBinding::new("enter", Enter, Some(KEY_CONTEXT)),
        KeyBinding::new("up", Up, Some(KEY_CONTEXT)),
        KeyBinding::new("down", Down, Some(KEY_CONTEXT)),
    ]);
}

pub struct Selector<D: SelectorDelegate> {
    delegate: D,

    id: ElementId,
    focus_handle: FocusHandle,
    bounds: Bounds<Pixels>,

    items: Vec<D::Item>,
    selected_ix: Option<usize>,
    open: bool,
}

impl<D: SelectorDelegate + 'static> Selector<D> {
    pub fn build(delegate: D, id: impl Into<ElementId>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            items: delegate.items(cx),
            delegate,

            id: id.into(),
            focus_handle: cx.focus_handle().clone(),
            bounds: Bounds::default(),

            selected_ix: None,
            open: false,
        })
    }

    pub fn set_selected_item(&mut self, item: Option<&D::Item>) {
        self.selected_ix = item.and_then(|item| self.items.iter().position(|i| i == item));
    }

    pub fn selected_item(&self) -> Option<&D::Item> {
        self.items.get(self.selected_ix?)
    }

    pub fn toggle_menu(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        self.open = !self.open;
        cx.notify();
    }

    fn escape(&mut self, _event: &Escape, cx: &mut ViewContext<Self>) {
        self.open = false;
        cx.notify();
    }

    fn enter(&mut self, _event: &Enter, cx: &mut ViewContext<Self>) {
        self.open = false;
        cx.emit(SelectorEvent::Change(self.selected_item().cloned()));
        cx.notify();
    }

    fn up(&mut self, _event: &Up, cx: &mut ViewContext<Self>) {
        eprintln!("TODO: Go up");
        cx.notify();
    }

    fn down(&mut self, _event: &Down, cx: &mut ViewContext<Self>) {
        eprintln!("TODO: Go down");
        cx.notify();
    }
}

impl<D: SelectorDelegate + 'static> Render for Selector<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let display_label = self
            .delegate
            .render_display_label(self.selected_item(), cx)
            .into_element();

        let list = uniform_list(
            cx.view().clone(),
            "selector-list",
            self.delegate.len(cx),
            |this, visible_range, cx| {
                visible_range
                    .map(|ix| {
                        let item = &this.items[ix];
                        let content = this.delegate.render_item(item, cx).into_element();
                        InteractiveContainer::new(
                            ContainerKind::Custom {
                                bg: ContainerKind::Element.bg(cx),
                                border_color: transparent_white(),
                            },
                            "list-button",
                            false,
                            false,
                        )
                        .w_full()
                        .h(cx.line_height())
                        .child(div().size_full().h_flex().p_1().child(content))
                        .border_b_1()
                        .border_color(cx.theme().border_variant)
                        .on_mouse_up(
                            MouseButton::Left,
                            cx.listener(move |this, _event, cx| {
                                this.selected_ix = Some(ix);
                                cx.dispatch_action(Box::new(Enter));
                            }),
                        )
                    })
                    .collect()
            },
        )
        .h(cx.line_height() * 4)
        .w_full();

        div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .key_context(KEY_CONTEXT)
            .on_action(cx.listener(Self::enter))
            .on_action(cx.listener(Self::escape))
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down))
            .w_full()
            .relative()
            .child(
                InteractiveContainer::new(ContainerKind::Element, "button", false, false)
                    .size_full()
                    .child(
                        div()
                            .relative()
                            .flex()
                            .items_center()
                            .justify_between()
                            .rounded(cx.theme().radius)
                            .overflow_hidden()
                            .w_full()
                            .on_mouse_down(MouseButton::Left, cx.listener(Self::toggle_menu))
                            .child(
                                div()
                                    .h_flex()
                                    .w_full()
                                    .items_center()
                                    .justify_between()
                                    .gap_1()
                                    .child(display_label),
                            ),
                    ),
            )
            .child(div().absolute().size_full().child(bounds_updater(
                cx.view().clone(),
                |this: &mut Self, bounds, _cx| this.bounds = bounds,
            )))
            .when(self.open, |this| {
                this.child(
                    deferred(
                        anchored().snap_to_window_with_margin(px(8.0)).child(
                            div()
                                .occlude()
                                .w(self.bounds.size.width)
                                .child(
                                    div()
                                        .v_flex()
                                        .occlude()
                                        .mt_px()
                                        .bg(cx.theme().background)
                                        .border_1()
                                        .border_color(cx.theme().border)
                                        .rounded(cx.theme().radius)
                                        .shadow_md()
                                        .on_mouse_down_out(|_, cx| {
                                            cx.dispatch_action(Box::new(Escape));
                                        })
                                        .child(list),
                                )
                                .on_mouse_down_out(|_, cx| {
                                    cx.dispatch_action(Box::new(Escape));
                                }),
                        ),
                    )
                    .with_priority(1),
                )
            })
    }
}

impl<D: SelectorDelegate + 'static> EventEmitter<SelectorEvent<D>> for Selector<D> {}

pub enum SelectorEvent<D: SelectorDelegate> {
    Change(Option<D::Item>),
}

pub trait SelectorDelegate {
    type Item: Clone + PartialEq;

    fn len(&self, cx: &ViewContext<Selector<Self>>) -> usize
    where
        Self: Sized;

    fn items(&self, cx: &ViewContext<Selector<Self>>) -> Vec<Self::Item>
    where
        Self: Sized;

    fn render_item(
        &self,
        item: &Self::Item,
        cx: &mut ViewContext<Selector<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;

    fn render_display_label(
        &self,
        item: Option<&Self::Item>,
        cx: &mut ViewContext<Selector<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}
