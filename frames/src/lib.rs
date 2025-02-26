use gpui::*;
use ui::{theme::ActiveTheme, utils::z_stack};

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
        self.frames.push(cx.new(|_| FrameWrapper { container, frame, bounds }));
    }

    pub fn show_grid(&mut self, show: bool) {
        self.show_grid = show;
    }

    fn render_grid(&self) -> impl IntoElement {
        canvas(|_, _, _| {}, {
            let width = self.grid_size.width;
            let height = self.grid_size.height;
            let cell_size = self.cell_size;
            move |_, _, window, cx| {
                for x in 0..(width + 1) {
                    for y in 0..(height + 1) {
                        window.paint_quad(fill(
                            Bounds::centered_at(
                                point(x as f32 * cell_size, y as f32 * cell_size),
                                size(px(2.0), px(2.0)),
                            ),
                            cx.theme().grid_color,
                        ));
                    }
                }
            }
        })
    }
}

impl<F: Frame + 'static> Render for FrameContainer<F> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let grid = self.show_grid.then(|| self.render_grid().into_any_element());
        let frames = div().size_full().children(self.frames.clone()).into_any_element();
        z_stack([grid, Some(frames)].into_iter().flatten())
    }
}

pub struct FrameWrapper<F: Frame> {
    container: Entity<FrameContainer<F>>,

    frame: F,
    bounds: Bounds<u32>,
}

impl<F: Frame + 'static> Render for FrameWrapper<F> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cell_size = self.container.read(cx).cell_size;

        div()
            .left(px(self.bounds.origin.x as f32) * cell_size)
            .top(px(self.bounds.origin.y as f32) * cell_size)
            .w(px(self.bounds.size.width as f32) * cell_size)
            .h(px(self.bounds.size.height as f32) * cell_size)
            .child(self.frame.render(cx))
    }
}

pub trait Frame {
    fn render_content(&mut self, cx: &mut Context<FrameWrapper<Self>>) -> impl IntoElement
    where
        Self: Sized;

    fn render(&mut self, cx: &mut Context<FrameWrapper<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        div()
            .size_full()
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border_color)
            .rounded(cx.theme().radius)
            .child(self.render_content(cx))
    }
}
