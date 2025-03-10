use gpui::*;
use prelude::FluentBuilder;
use ui::{theme::ActiveTheme, z_stack};

pub struct FrameContainer<F: Frame> {
    grid_size: Size<u32>,
    cell_size: Pixels,

    show_grid: bool,

    frames: Vec<Entity<FrameWrapper<F>>>,
}

impl<F: Frame + 'static> FrameContainer<F> {
    pub fn new(grid_size: Size<u32>, cell_size: Pixels) -> Self {
        Self { grid_size, cell_size, show_grid: true, frames: Vec::new() }
    }

    pub fn add_frame(&mut self, frame: F, bounds: Bounds<u32>, cx: &mut Context<Self>) {
        let container = cx.entity().clone();
        self.frames.push(cx.new(|cx| FrameWrapper {
            container,
            frame,
            bounds,
            focus_handle: cx.focus_handle(),
        }));
    }

    pub fn show_grid(&mut self, show: bool) {
        self.show_grid = show;
    }
}

impl<F: Frame + 'static> Render for FrameContainer<F> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let grid = self.show_grid.then(|| {
            ui::dot_grid(self.cell_size, cx.theme().grid_color)
                .w(self.grid_size.width as f32 * self.cell_size)
                .h(self.grid_size.height as f32 * self.cell_size)
                .into_any_element()
        });
        let frames = div().size_full().children(self.frames.clone()).into_any_element();
        z_stack([grid, Some(frames)].into_iter().flatten())
    }
}

pub struct FrameWrapper<F: Frame> {
    container: Entity<FrameContainer<F>>,

    frame: F,
    bounds: Bounds<u32>,

    focus_handle: FocusHandle,
}

impl<F: Frame> FrameWrapper<F> {
    pub fn frame(&self) -> &F {
        &self.frame
    }

    pub fn bounds(&self) -> &Bounds<u32> {
        &self.bounds
    }
}

impl<F: Frame + 'static> Render for FrameWrapper<F> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cell_size = self.container.read(cx).cell_size;

        let focused = self.focus_handle.is_focused(window);

        div()
            .track_focus(&self.focus_handle)
            .left(px(self.bounds.origin.x as f32) * cell_size)
            .top(px(self.bounds.origin.y as f32) * cell_size)
            .w(px(self.bounds.size.width as f32) * cell_size)
            .h(px(self.bounds.size.height as f32) * cell_size)
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border_color)
            .when(focused, |e| e.border_color(cx.theme().border_color_focused))
            .rounded(cx.theme().radius)
            .child(self.frame.render(window, cx))
    }
}

impl<F: Frame + 'static> Focusable for FrameWrapper<F> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub trait Frame {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<FrameWrapper<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}
