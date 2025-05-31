use super::Frame;
use crate::{
    show,
    ui::{FRAME_CELL_SIZE, FrameSelector},
};
use gpui::{
    App, Bounds, DismissEvent, Entity, EntityId, MouseButton, MouseDownEvent, Pixels, Point, Size,
    Window, anchored, bounds, deferred, div, prelude::*, size,
};
use ui::{
    ActiveTheme,
    utils::{bounds_updater, z_stack},
};

pub const GRID_SIZE: Size<u32> = Size { width: 18, height: 12 };

#[derive(Debug, Clone)]
pub struct Page {
    label: String,
    frames: Vec<Entity<Frame>>,
    frame_seletor: Option<(Entity<FrameSelector>, Point<Pixels>)>,
    bounds: Bounds<Pixels>,
}

impl Page {
    pub fn new(label: String) -> Self {
        Self { label, frames: Vec::new(), frame_seletor: None, bounds: Bounds::default() }
    }

    pub fn add_frame(&mut self, frame: Entity<Frame>, cx: &mut Context<Self>) {
        cx.observe(&frame, |_, _, cx| {
            cx.notify();
        })
        .detach();

        self.frames.push(frame);

        cx.notify();
    }

    pub fn remove_frame(&mut self, frame_id: EntityId, cx: &mut Context<Self>) {
        self.frames.retain(|frame| frame.entity_id() != frame_id);
        cx.notify();
    }

    fn deploy_frame_selector(
        &mut self,
        position: Point<Pixels>,
        frame_bounds: Bounds<u32>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let page = cx.entity();
        let frame_selector = cx.new(|cx| FrameSelector::new(page, frame_bounds, window, cx));

        cx.subscribe(&frame_selector, |this, _, _: &DismissEvent, cx| {
            this.frame_seletor = None;
            cx.notify();
        })
        .detach();

        self.frame_seletor = Some((frame_selector, position));
        cx.notify();
    }
}

impl Render for Page {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let grid = ui::dot_grid(FRAME_CELL_SIZE, cx.theme().colors.grid)
            .w(GRID_SIZE.width as f32 * FRAME_CELL_SIZE)
            .h(GRID_SIZE.height as f32 * FRAME_CELL_SIZE);

        let background = z_stack([
            grid.into_any_element(),
            div()
                .size_full()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, event: &MouseDownEvent, window, cx| {
                        cx.stop_propagation();
                        let position = event.position - this.bounds.origin;
                        let frame_bounds =
                            bounds(position.map(|d| (d / FRAME_CELL_SIZE) as u32), size(1, 1));
                        this.deploy_frame_selector(event.position, frame_bounds, window, cx);
                        cx.notify();
                    }),
                )
                .into_any_element(),
        ])
        .size_full();

        let frames = z_stack(self.frames.clone()).size_full();

        div()
            .size_full()
            .child(
                z_stack([
                    background.into_any_element(),
                    frames.into_any_element(),
                    bounds_updater(cx.entity(), |this, bounds, _cx| this.bounds = bounds)
                        .into_any_element(),
                ])
                .size_full(),
            )
            .children(self.frame_seletor.as_ref().map(|(frame_selector, position)| {
                deferred(
                    anchored().position(*position).child(
                        div()
                            .w_128()
                            .h_72()
                            .on_mouse_down_out(cx.listener(|this, _, _window, cx| {
                                this.frame_seletor = None;
                                cx.stop_propagation();
                                cx.notify();
                            }))
                            .child(frame_selector.clone()),
                    ),
                )
                .with_priority(1)
            }))
    }
}

impl Page {
    pub fn into_show(&self, cx: &App) -> show::Page {
        show::Page {
            label: self.label.clone(),
            frames: self
                .frames
                .clone()
                .into_iter()
                .map(|frame| frame.read(cx).into_show(cx))
                .collect(),
        }
    }

    pub fn from_show(
        layout: &Entity<show::Layout>,
        window: &mut Window,
        cx: &mut Context<Page>,
    ) -> Self {
        let loaded_page = &layout.read(cx).main_window.loaded_page;
        let mut page = Self::new(loaded_page.label.clone());

        for frame in &loaded_page.frames.clone() {
            let page_view = cx.entity();
            let frame = cx.new(|cx| Frame::from_show(frame, cx.entity(), page_view, window, cx));

            cx.observe(&frame, |_, _, cx| {
                cx.notify();
            })
            .detach();

            page.add_frame(frame, cx);
        }

        page
    }
}
