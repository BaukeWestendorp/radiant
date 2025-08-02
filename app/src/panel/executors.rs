use gpui::prelude::*;
use gpui::{Entity, FontWeight, ReadGlobal, Window, div};
use radiant::engine::Command;
use radiant::show::{Executor, Object, ObjectId};
use ui::{ActiveTheme, ContainerStyle, button, container};

use crate::app::{AppState, with_show};
use crate::main_window::CELL_SIZE;

pub struct ExecutorsPanel {
    executors: Vec<Entity<ExecutorView>>,
}

impl ExecutorsPanel {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            executors: vec![
                cx.new(|_| ExecutorView::new(Some(1.into()))),
                cx.new(|_| ExecutorView::new(None)),
            ],
        }
    }
}

impl Render for ExecutorsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().flex().children(self.executors.clone())
    }
}

struct ExecutorView {
    executor_id: Option<ObjectId<Executor>>,
}

impl ExecutorView {
    pub fn new(executor_id: Option<ObjectId<Executor>>) -> Self {
        Self { executor_id }
    }
}

impl Render for ExecutorView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(executor) =
            self.executor_id.and_then(|id| with_show(cx, |show| show.executors.get(id).cloned()))
        else {
            return container(ContainerStyle::normal(window, cx))
                .size(CELL_SIZE)
                .into_any_element();
        };

        let name = div()
            .border_b_1()
            .border_color(cx.theme().colors.border)
            .font_weight(FontWeight::BOLD)
            .child(executor.name().to_string());

        let sequence_content = match with_show(cx, |show| executor.sequence(show).cloned()) {
            Some(sequence) => {
                let prev_cue = sequence.previous_cue().map(|cue| cue.name()).unwrap_or_default();
                let active_cue = sequence.active_cue().map(|cue| cue.name()).unwrap_or_default();
                let next_cue = sequence.next_cue().map(|cue| cue.name()).unwrap_or_default();

                div()
                    .child(
                        div()
                            .border_b_1()
                            .border_color(cx.theme().colors.border)
                            .child(prev_cue.to_string()),
                    )
                    .child(
                        div()
                            .bg(cx.theme().colors.bg_selected_bright)
                            .border_b_1()
                            .border_color(cx.theme().colors.border)
                            .child(active_cue.to_string()),
                    )
                    .child(
                        div()
                            .border_b_1()
                            .border_color(cx.theme().colors.border)
                            .child(next_cue.to_string()),
                    )
            }
            None => div(),
        };

        div()
            .flex()
            .flex_col()
            .child(
                container(ContainerStyle::normal(window, cx))
                    .size(CELL_SIZE)
                    .text_xs()
                    .child(name)
                    .child(sequence_content),
            )
            .child(button("button_1", None, "Go").on_click({
                let executor_id = executor.id();
                move |_, _, cx| {
                    AppState::global(cx)
                        .engine
                        .exec(Command::Go { executor: executor_id })
                        .unwrap();
                }
            }))
            .into_any_element()
    }
}
