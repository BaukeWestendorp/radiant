use gpui::{
    AnyElement, App, Bounds, DismissEvent, Div, DragMoveEvent, Empty, Entity, EntityId,
    FocusHandle, Focusable, MouseButton, MouseDownEvent, Path, Pixels, Point, Stateful,
    Subscription, Window, anchored, canvas, deferred, div, point, prelude::*, px,
};
use pool::{PoolFrame, PoolFrameKind};
use ui::utils::z_stack;
use ui::{ActiveTheme, ContainerStyle, ContextMenu, container};
use window::{WindowFrame, WindowFrameKind};

use super::Page;
use crate::show;
use crate::ui::FRAME_CELL_SIZE;

mod pool;
mod window;

pub struct Frame {
    pub page: Entity<Page>,

    pub bounds: Bounds<u32>,
    pub kind: FrameKind,
    focus_handle: FocusHandle,

    header_context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
}

impl Frame {
    pub fn new(kind: FrameKind, bounds: Bounds<u32>, page: Entity<Page>, cx: &mut App) -> Self {
        Self { page, kind, bounds, focus_handle: cx.focus_handle(), header_context_menu: None }
    }

    fn render_resize_handle(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let frame_bounds = self.bounds;

        div()
            .id("resize_handle")
            .absolute()
            .right_0()
            .bottom_0()
            .size_4()
            .child(
                canvas(
                    |_, _, _| {},
                    |b, _, w, cx| {
                        let b = b + point(px(-1.0), px(-1.0));
                        let mut path = Path::new(b.bottom_left());
                        path.line_to(b.top_right());
                        path.line_to(b.bottom_right());
                        path.line_to(b.bottom_left());
                        w.paint_path(path, cx.theme().colors.border);
                    },
                )
                .size_full(),
            )
            .occlude()
            .on_mouse_down(MouseButton::Left, {
                let frame = cx.entity();
                let page = self.page.clone();
                move |event: &MouseDownEvent, _, cx| {
                    page.update(cx, |page, cx| {
                        page.start_resize_frame(
                            frame_bounds,
                            frame.entity_id(),
                            event.position,
                            cx,
                        );
                    });
                }
            })
            .on_drag(cx.entity_id(), |_, _, _, cx| cx.new(|_| Empty))
            .on_drag_move({
                let frame = cx.entity();
                let page = self.page.clone();
                move |event: &DragMoveEvent<EntityId>, _, cx| {
                    if event.drag(cx) != &frame.entity_id() {
                        return;
                    }

                    page.update(cx, |page, cx| {
                        page.drag_resize_frame(event.event.position, cx);
                    });
                }
            })
            .on_mouse_up(MouseButton::Left, {
                let page = self.page.clone();
                let frame = cx.entity();
                move |_, _, cx| {
                    page.update(cx, |page, cx| {
                        page.end_resize_frame(&frame, cx);
                    });
                }
            })
            .on_mouse_up_out(MouseButton::Left, {
                let page = self.page.clone();
                let frame = cx.entity();
                move |_, _, cx| {
                    page.update(cx, |page, cx| {
                        page.end_resize_frame(&frame, cx);
                    });
                }
            })
            .cursor_nwse_resize()
    }

    fn render_frame_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match &self.kind {
            FrameKind::Window(window_frame) => window_frame.clone().into_any_element(),
            FrameKind::Pool(pool_frame) => pool_frame.clone().into_any_element(),
        }
    }
}

impl Frame {
    pub fn handle_remove(
        &mut self,
        _: &actions::Remove,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let frame_id = cx.entity_id();
        self.page.update(cx, |page, cx| {
            page.remove_frame(frame_id, cx);
            cx.notify();
        })
    }

    pub fn open_header_context_menu(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_menu = cx.new(|cx| {
            ContextMenu::new(window, cx)
                .destructive_action("Remove Frame", Box::new(actions::Remove))
        });

        let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
            this.header_context_menu.take();
            cx.notify();
        });

        self.header_context_menu = Some((context_menu, event.position, subscription));
    }
}

impl Render for Frame {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let resize_handle = self.render_resize_handle(window, cx).into_any_element();
        let frame_content = self.render_frame_content(window, cx).into_any_element();

        div()
            .track_focus(&self.focus_handle)
            .absolute()
            .w(FRAME_CELL_SIZE * self.bounds.size.width as f32)
            .h(FRAME_CELL_SIZE * self.bounds.size.height as f32)
            .left(FRAME_CELL_SIZE * self.bounds.origin.x as f32)
            .top(FRAME_CELL_SIZE * self.bounds.origin.y as f32)
            .occlude()
            .on_action(cx.listener(Self::handle_remove))
            .child(z_stack([frame_content, resize_handle]).size_full())
            .children(self.header_context_menu.as_ref().map(|(menu, position, _)| {
                deferred(anchored().position(*position).child(menu.clone())).with_priority(1)
            }))
    }
}

impl Frame {
    pub fn into_show(&self, cx: &App) -> show::Frame {
        show::Frame { bounds: self.bounds, kind: self.kind.into_show(cx) }
    }

    pub fn from_show(
        from: &show::Frame,
        frame: Entity<Frame>,
        page: Entity<Page>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self::new(FrameKind::from_show(&from.kind, frame, window, cx), from.bounds, page, cx)
    }
}

impl Focusable for Frame {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

pub enum FrameKind {
    Window(Entity<WindowFrame>),
    Pool(Entity<PoolFrame>),
}

impl FrameKind {
    pub fn into_show(&self, cx: &App) -> show::FrameKind {
        match self {
            FrameKind::Window(window_frame) => {
                show::FrameKind::Window(window_frame.read(cx).kind.into_show(cx))
            }
            FrameKind::Pool(pool_frame) => {
                show::FrameKind::Pool(pool_frame.read(cx).kind.into_show())
            }
        }
    }

    pub fn from_show(
        from: &show::FrameKind,
        frame: Entity<Frame>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match from {
            show::FrameKind::Window(kind) => {
                let kind = WindowFrameKind::from_show(kind, window, cx);
                let window_frame = cx.new(|_| WindowFrame::new(kind, frame));
                Self::Window(window_frame)
            }
            show::FrameKind::Pool(kind) => {
                let pool_frame_kind = PoolFrameKind::from_show(kind);
                let pool_frame = cx.new(|_| PoolFrame::new(pool_frame_kind, frame));
                FrameKind::Pool(pool_frame)
            }
        }
    }
}

fn header_container(
    frame: Entity<Frame>,
    content: AnyElement,
    window: &Window,
    cx: &App,
) -> Stateful<Div> {
    let border_color = if frame.focus_handle(cx).contains_focused(window, cx) {
        cx.theme().colors.border_focused
    } else {
        cx.theme().colors.header_border
    };

    div()
        .child(
            container(ContainerStyle {
                background: cx.theme().colors.header_background,
                border: border_color,
                text_color: cx.theme().colors.text,
            })
            .size_full()
            .child(content),
        )
        .id("frame_header")
        .cursor_grab()
        .on_mouse_down(MouseButton::Left, {
            let frame = frame.clone();
            move |event: &MouseDownEvent, _, cx| {
                frame.read(cx).page.clone().update(cx, {
                    |page, cx| {
                        page.start_move_frame(&frame, event.position, cx);
                    }
                });
            }
        })
        .on_drag(frame.entity_id(), |_, _, _, cx| cx.new(|_| Empty))
        .on_drag_move({
            let frame = frame.clone();
            move |event: &DragMoveEvent<EntityId>, _, cx| {
                if event.drag(cx) != &frame.entity_id() {
                    return;
                }

                frame
                    .read(cx)
                    .page
                    .clone()
                    .update(cx, |page, cx| page.drag_move_frame(event.event.position, cx));
            }
        })
        .on_mouse_up(MouseButton::Left, {
            let frame = frame.clone();
            move |_, _, cx| {
                frame.read(cx).page.clone().update(cx, |page, cx| {
                    page.end_move_frame(&frame, cx);
                });
            }
        })
        .on_mouse_up_out(MouseButton::Left, {
            let frame = frame.clone();
            move |_, _, cx| {
                frame.read(cx).page.clone().update(cx, |page, cx| {
                    page.end_move_frame(&frame, cx);
                });
            }
        })
        .on_mouse_down(MouseButton::Right, move |event, w, cx| {
            frame.update(cx, |frame, cx| frame.open_header_context_menu(event, w, cx));
        })
}

pub mod actions {
    use gpui::actions;

    actions!(new_node_menu, [Remove]);
}
