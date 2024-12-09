use gpui::*;

use crate::{theme::ActiveTheme, ContainerKind, InteractiveContainer};

#[derive(IntoElement)]
pub struct Button {
    kind: ButtonKind,
    label: SharedString,
    id: ElementId,
    disabled: bool,
    focused: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Button {
    pub fn new(kind: ButtonKind, label: impl Into<SharedString>, id: impl Into<ElementId>) -> Self {
        Self {
            kind,
            label: label.into(),
            id: id.into(),
            focused: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }
}

impl From<Button> for AnyElement {
    fn from(button: Button) -> Self {
        button.into_any_element()
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        InteractiveContainer::new(
            ContainerKind::Custom {
                bg: self.kind.bg(cx),
                border_color: self.kind.border_color(cx),
            },
            self.id,
            self.disabled,
            self.focused,
        )
        .w_full()
        .h(cx.theme().input_height)
        .on_click(move |event, cx| {
            if let Some(on_click) = &self.on_click {
                on_click(event, cx);
            }
        })
        .child(
            div()
                .size_full()
                .text_color(self.kind.text_color(cx))
                .flex()
                .items_center()
                .px_1()
                .text_xs()
                .child(self.label),
        )
    }
}

pub enum ButtonKind {
    Primary,
    Ghost,
    Custom {
        bg: Hsla,
        border_color: Hsla,
        text_color: Hsla,
    },
}

impl ButtonKind {
    fn text_color(&self, cx: &WindowContext) -> Hsla {
        match self {
            ButtonKind::Primary => cx.theme().text,
            ButtonKind::Ghost => cx.theme().text,
            ButtonKind::Custom { text_color, .. } => *text_color,
        }
    }

    fn bg(&self, cx: &WindowContext) -> Hsla {
        match self {
            ButtonKind::Primary => cx.theme().element_background,
            ButtonKind::Ghost => cx.theme().background,
            ButtonKind::Custom { bg, .. } => *bg,
        }
    }

    fn border_color(&self, cx: &WindowContext) -> Hsla {
        match self {
            ButtonKind::Primary => cx.theme().border,
            ButtonKind::Ghost => transparent_white(),
            ButtonKind::Custom { border_color, .. } => *border_color,
        }
    }
}
