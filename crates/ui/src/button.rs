use gpui::*;

use crate::theme::ActiveTheme;

pub fn button(label: SharedString, id: impl Into<ElementId>, cx: &AppContext) -> Stateful<Div> {
    div()
        .id(id.into())
        .h(cx.theme().input_height)
        .flex()
        .items_center()
        .bg(cx.theme().element_background)
        .min_w_10()
        .border_1()
        .border_color(cx.theme().border)
        .rounded(cx.theme().radius)
        .hover(|e| e.bg(cx.theme().element_hover))
        .active(|e| e.bg(cx.theme().element_active))
        .cursor_pointer()
        .px_1()
        .child(label)
}
