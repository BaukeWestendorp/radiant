use gpui::*;
use ui::{theme::ActiveTheme, z_stack};

use crate::showfile::Showfile;

pub const GRID_SIZE: f32 = 80.0;

pub struct FrameGrid {
    width: u32,
    height: u32,

    frames: Vec<AnyView>,
}

impl FrameGrid {
    pub fn build(cx: &mut WindowContext) -> View<FrameGrid> {
        cx.new_view(|cx| {
            let frames = Showfile::global(cx)
                .windows()
                .main_window
                .frames
                .clone()
                .into_iter()
                .map(|kind| kind.to_frame_view(cx))
                .collect();

            FrameGrid {
                width: 16,
                height: 10,

                frames,
            }
        })
    }
}

impl Render for FrameGrid {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        let background = canvas(|_, _| {}, {
            let width = self.width;
            let height = self.height;
            move |_, _, cx| {
                for x in 0..(width + 1) {
                    for y in 0..(height + 1) {
                        cx.paint_quad(fill(
                            Bounds::centered_at(
                                point(px(x as f32 * GRID_SIZE), px(y as f32 * GRID_SIZE)),
                                size(px(2.0), px(2.0)),
                            ),
                            cx.theme().accent.opacity(0.5),
                        ));
                    }
                }
            }
        })
        .size_full()
        .into_any_element();

        let frame_elements = self.frames.clone().into_iter().map(|frame| {
            div()
                .absolute()
                .left(px(0 as f32 * GRID_SIZE))
                .top(px(0 as f32 * GRID_SIZE))
                .w(px(15 as f32 * GRID_SIZE))
                .h(px(8 as f32 * GRID_SIZE))
                .child(frame)
                .into_any_element()
        });

        z_stack([background].into_iter().chain(frame_elements)).size_full()
    }
}
