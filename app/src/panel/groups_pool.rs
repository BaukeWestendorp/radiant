use gpui::prelude::*;
use gpui::{Size, Window, div};
use radiant::show::{Group, Object, ObjectId};
use ui::{ActiveTheme, ContainerStyle, container};

use crate::app::with_show;
use crate::main_window::CELL_SIZE;

pub struct GroupsPool {
    size: Size<u32>,
}

impl GroupsPool {
    pub fn new(size: Size<u32>) -> Self {
        Self { size }
    }
}

impl Render for GroupsPool {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header_cell = container(ContainerStyle {
            background: cx.theme().colors.header_background,
            border: cx.theme().colors.header_border,
            text_color: window.text_style().color,
        })
        .size_full();

        let area = self.size.width * self.size.height;
        let mut pool_cells = vec![header_cell.into_any_element()];
        for ix in 1..area {
            let id = ObjectId::<Group>::new(ix);
            let group = with_show(cx, |show| show.groups.get(id).cloned());

            let cell = if let Some(group) = group {
                container(ContainerStyle::normal(window, cx))
                    .size_full()
                    .child(id.to_string())
                    .child(group.name().to_string())
            } else {
                container(ContainerStyle::normal(window, cx).disabled())
                    .size_full()
                    .child(id.to_string())
            }
            .into_any_element();
            pool_cells.push(cell);
        }

        div()
            .flex()
            .flex_wrap()
            .size_full()
            .children(pool_cells.into_iter().map(|cell| div().size(CELL_SIZE).p_px().child(cell)))
    }
}
