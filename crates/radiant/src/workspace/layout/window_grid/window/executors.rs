use backstage::show::{Cue, Executor, Sequence, Show};
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, uniform_list, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use super::{WindowDelegate, WindowView};
use crate::theme::ActiveTheme;
use crate::workspace::layout::window_grid::{grid_div, GridSize, GRID_CELL_SIZE};

pub struct ExecutorsWindowDelegate {
    executors: Vec<Executor>,
    show: Model<Show>,
}

impl ExecutorsWindowDelegate {
    pub fn new(show: Model<Show>, cx: &mut WindowContext) -> Self {
        let executors = show.read(cx).executors().clone();
        Self { executors, show }
    }
}

impl WindowDelegate for ExecutorsWindowDelegate {
    fn title(&self) -> String {
        "Executors".to_string()
    }

    fn render_content(&self, cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        let executor_views = self
            .executors
            .iter()
            .map(|executor| ExecutorView::build(executor.clone(), self.show.clone(), cx))
            .collect::<Vec<_>>();

        div().size_full().flex().children(executor_views)
    }
}

pub struct ExecutorView {
    info: View<ExecutorInfo>,
    button_1: View<ExecutorButton>,
    button_2: View<ExecutorButton>,
    button_3: View<ExecutorButton>,
}

impl ExecutorView {
    pub fn build(executor: Executor, show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            info: ExecutorInfo::build(executor, show, cx),
            button_1: ExecutorButton::build(cx),
            button_2: ExecutorButton::build(cx),
            button_3: ExecutorButton::build(cx),
        })
    }
}

impl Render for ExecutorView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .justify_between()
            .child(self.info.clone())
            .child(
                div()
                    .child(self.button_1.clone())
                    .child(self.button_2.clone())
                    .child(self.button_3.clone()),
            )
    }
}

pub struct ExecutorInfo {
    executor: Executor,
    show: Model<Show>,
}

impl ExecutorInfo {
    pub fn build(executor: Executor, show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { executor, show })
    }

    fn render_header(
        &self,
        sequence: Option<&Sequence>,
        cx: &mut WindowContext,
    ) -> impl IntoElement {
        div()
            .w_full()
            .h_5()
            .border_b()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().elevated_surface_background)
            .flex()
            .items_center()
            .px_1()
            .text_xs()
            .child(
                sequence
                    .map(|s| s.label.clone())
                    .unwrap_or("Empty".to_string()),
            )
            .when(sequence.is_none(), |this| {
                this.text_color(cx.theme().colors().text_disabled)
            })
    }

    fn render_cues(
        &self,
        sequence: Option<&Sequence>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let cues = sequence.map(|s| s.cues.clone()).unwrap_or_default();
        uniform_list(
            cx.view().clone(),
            "cues",
            cues.len(),
            move |this, range, cx| {
                let end = range.end;
                cues[range]
                    .iter()
                    .enumerate()
                    .map(|(ix, cue)| {
                        let active = this.executor.current_index.get() == Some(ix);
                        let last = end - 1 == ix;
                        this.render_cue(cue, active, last, cx)
                    })
                    .collect()
            },
        )
    }

    fn render_cue(
        &self,
        cue: &Cue,
        active: bool,
        last: bool,
        cx: &mut WindowContext,
    ) -> impl IntoElement {
        div()
            .text_xs()
            .px_1()
            .when(!last, |this| this.border_b())
            .border_color(cx.theme().colors().border)
            .child(cue.label.clone())
            .when(active, |this| {
                this.bg(cx.theme().colors().background_selected)
            })
    }
}

impl Render for ExecutorInfo {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let sequence = self
            .executor
            .sequence
            .and_then(|id| self.show.read(cx).get_sequence(id).cloned());

        grid_div(GridSize::new(1, 1), None)
            .bg(cx.theme().colors().background)
            .border()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(self.render_header(sequence.as_ref(), cx))
            .child(self.render_cues(sequence.as_ref(), cx))
    }
}

pub struct ExecutorButton {}

impl ExecutorButton {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }
}

impl Render for ExecutorButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(px(GRID_CELL_SIZE as f32))
            .h(px(GRID_CELL_SIZE as f32 / 2.0))
            .bg(cx.theme().colors().background)
            .border()
            .border_color(cx.theme().colors().border)
            .rounded_md()
    }
}
