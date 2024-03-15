use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, rgb, uniform_list, white, IntoElement, Model, ParentElement, Render, Styled, View,
    ViewContext, VisualContext,
};

use crate::show::Show;

use super::Window;

pub struct FixtureSheetWindow {
    show: Model<Show>,
}

impl FixtureSheetWindow {
    pub fn build(show: Model<Show>, cx: &mut ViewContext<Window>) -> View<Self> {
        cx.new_view(|_cx| Self { show })
    }
}

impl Render for FixtureSheetWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let fixtures = self.show.read(cx).clone().patch.fixtures;

        uniform_list(
            cx.view().clone(),
            "fixture_list",
            fixtures.len(),
            move |view, _visible_range, cx| {
                let mut rows = vec![];

                let column_width = px(130.0);

                let header_row = div()
                    .flex()
                    .flex_row()
                    .child(
                        div()
                            .child("ID")
                            .w(column_width)
                            .h(cx.line_height())
                            .overflow_hidden()
                            .border_color(white())
                            .border_r(),
                    )
                    .child(
                        div()
                            .child("Name")
                            .overflow_hidden()
                            .w(column_width)
                            .h(cx.line_height())
                            .border_color(white())
                            .border_r(),
                    )
                    .child(
                        div()
                            .child("Mode")
                            .overflow_hidden()
                            .w(column_width)
                            .h(cx.line_height())
                            .border_color(white())
                            .border_r(),
                    )
                    .child(
                        div()
                            .child("Address")
                            .overflow_hidden()
                            .w(column_width)
                            .h(cx.line_height())
                            .border_color(white())
                            .border_r(),
                    )
                    .child(
                        div()
                            .child("Universe")
                            .overflow_hidden()
                            .w(column_width)
                            .h(cx.line_height())
                            .border_color(white())
                            .border_r(),
                    );

                rows.push(header_row);

                let patch = &view.show.read(cx).patch;
                for (ix, fixture) in fixtures.iter().enumerate() {
                    let row = div()
                        .flex()
                        .flex_row()
                        .when(ix % 2 == 0, |this| this.bg(rgb(0x444444)))
                        .child(
                            div()
                                .child(format!("{}", fixture.id))
                                .overflow_hidden()
                                .w(column_width)
                                .h(cx.line_height())
                                .border_color(white())
                                .border_r(),
                        )
                        .child(
                            div()
                                .child(format!("{}", fixture.name))
                                .overflow_hidden()
                                .w(column_width)
                                .h(cx.line_height())
                                .border_color(white())
                                .border_r(),
                        )
                        .child(
                            div()
                                .child(format!(
                                    "Mode {} ({} channels)",
                                    fixture.mode_index,
                                    fixture.get_valid_channels(&patch).len()
                                ))
                                .overflow_hidden()
                                .w(column_width)
                                .h(cx.line_height())
                                .border_color(white())
                                .border_r(),
                        )
                        .child(
                            div()
                                .child(format!("{}", fixture.address))
                                .overflow_hidden()
                                .w(column_width)
                                .h(cx.line_height())
                                .border_color(white())
                                .border_r(),
                        )
                        .child(
                            div()
                                .child(format!("{}", fixture.universe))
                                .overflow_hidden()
                                .w(column_width)
                                .h(cx.line_height())
                                .border_color(white())
                                .border_r(),
                        );

                    rows.push(row);
                }

                rows
            },
        )
    }
}
