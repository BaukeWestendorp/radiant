use gpui::{
    App, Bounds, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Window, div,
    prelude::*,
};
use ui::{
    ActiveTheme, ContainerStyle, Field, FieldEvent, InteractiveColor, List, SubmitEvent, container,
};

use crate::{
    layout::{Frame, FrameKind, Page},
    show,
};

pub struct FrameSelector {
    page: Entity<Page>,

    search_field: Entity<Field<String>>,
    list: Entity<List<show::FrameKind>>,
    focus_handle: FocusHandle,
}

impl FrameSelector {
    pub fn new(
        page: Entity<Page>,
        frame_bounds: Bounds<u32>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        let search_field = cx.new(|cx| {
            let field = Field::new("search_field", focus_handle.clone(), window, cx);
            field.set_placeholder("Search...", cx);
            field
        });

        cx.subscribe(&search_field, {
            let focus_handle = focus_handle.clone();
            move |this: &mut Self, _, event: &FieldEvent<String>, cx| match event {
                ui::FieldEvent::Change(value) => this.list.update(cx, {
                    let focus_handle = focus_handle.clone();

                    move |list, cx| {
                        let filtered_items = show::FrameKind::all().into_iter().filter(|item| {
                            item.to_string().to_lowercase().contains(&value.to_lowercase())
                        });

                        *list = List::new("items", focus_handle, filtered_items, |kind| {
                            kind.to_string().into()
                        });
                        list.select_index(0, cx);
                        cx.notify();
                    }
                }),
                _ => {}
            }
        })
        .detach();

        let list = cx.new(|cx| {
            let mut list =
                List::new("items", focus_handle.clone(), show::FrameKind::all(), |kind| {
                    kind.to_string().into()
                });
            list.select_index(0, cx);
            list
        });

        cx.subscribe_in(
            &list,
            window,
            move |this: &mut Self, list, _event: &SubmitEvent, window, cx| {
                let Some(selected_frame_kind) = list.read(cx).selected_item().copied() else {
                    return;
                };

                let page_entity = this.page.clone();
                this.page.update(cx, |page, cx| {
                    let frame = cx.new(|cx| {
                        Frame::new(
                            FrameKind::from_show(&selected_frame_kind, cx.entity(), window, cx),
                            frame_bounds,
                            page_entity,
                            cx,
                        )
                    });
                    page.add_frame(frame, cx);
                });

                this.handle_dismiss(&actions::Dismiss, window, cx);
            },
        )
        .detach();

        Self { page, search_field, list, focus_handle }
    }
}

impl FrameSelector {
    fn handle_dismiss(
        &mut self,
        _: &actions::Dismiss,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DismissEvent);
    }
}

impl Render for FrameSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = div()
            .p_1()
            .w_full()
            .child(self.search_field.clone())
            .border_b_1()
            .border_color(cx.theme().colors.border.muted());

        container(ContainerStyle::normal(window, cx))
            .key_context(actions::KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .occlude()
            .on_action(cx.listener(Self::handle_dismiss))
            .child(div().size_full().flex().flex_col().child(header).child(self.list.clone()))
    }
}

impl Focusable for FrameSelector {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for FrameSelector {}

pub mod actions {
    use gpui::{App, KeyBinding, actions};

    pub const KEY_CONTEXT: &str = "FrameSelector";

    actions!(new_node_menu, [Dismiss]);

    pub fn init(cx: &mut App) {
        bind_keys(cx);
    }

    fn bind_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", Dismiss, Some(KEY_CONTEXT))]);
    }
}
