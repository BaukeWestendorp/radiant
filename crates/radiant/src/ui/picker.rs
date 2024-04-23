use gpui::{
    div, prelude::FluentBuilder, px, InteractiveElement, IntoElement, Model, ParentElement, Render,
    SharedString, StatefulInteractiveElement, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

pub struct Picker {
    labels: Vec<SharedString>,
    selected: Model<Option<usize>>,
}

impl Picker {
    pub fn build(
        labels: Vec<SharedString>,
        selected: Model<Option<usize>>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self { labels, selected })
    }
}

impl Render for Picker {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let selected_ix = self.selected.read(cx);

        let buttons = self
            .labels
            .clone()
            .into_iter()
            .enumerate()
            .map(|(ix, label)| {
                div()
                    .id(label.clone())
                    .px_2()
                    .py(px(2.0))
                    .border()
                    .border_color(gpui::white())
                    .on_click(cx.listener(move |this, _event, cx| {
                        this.selected.update(cx, |selected, cx| {
                            *selected = Some(ix);
                            cx.notify();
                        })
                    }))
                    .when(*selected_ix == Some(ix), |this| {
                        this.border_color(gpui::red())
                    })
                    .child(label)
            });

        div().children(buttons)
    }
}
