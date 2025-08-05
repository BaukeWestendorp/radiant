use std::num::NonZeroU32;

use gpui::prelude::*;
use gpui::{App, Entity, Window, div, relative};
use radiant::engine::Command;
use radiant::show::{Cue, Executor, Object, ObjectId, PoolId, Sequence};
use ui::utils::z_stack;
use ui::{ActiveTheme, ContainerStyle, container};

use crate::main_window::CELL_SIZE;
use crate::panel::window::{WindowPanel, WindowPanelDelegate};
use crate::state::{exec_cmd_and_log_err, with_show};

pub struct ExecutorsPanel {
    executors: Vec<Entity<ExecutorView>>,
}

impl ExecutorsPanel {
    pub fn new(columns: u32, _window: &mut Window, cx: &mut Context<WindowPanel<Self>>) -> Self {
        Self {
            executors: (1..columns + 1)
                .into_iter()
                .map(|ix| {
                    cx.new(|cx| {
                        let pool_id = PoolId::<Executor>::new(NonZeroU32::new(ix).unwrap());
                        let id = with_show(cx, |show| show.object_id_from_pool_id(pool_id));
                        ExecutorView::new(id)
                    })
                })
                .collect(),
        }
    }
}

impl WindowPanelDelegate for ExecutorsPanel {
    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowPanel<Self>>,
    ) -> impl IntoElement {
        div().size_full().flex().children(self.executors.clone())
    }
}

struct ExecutorView {
    executor_id: Option<ObjectId>,
}

impl ExecutorView {
    pub fn new(executor_id: Option<ObjectId>) -> Self {
        Self { executor_id }
    }
}

impl Render for ExecutorView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let executor =
            with_show(cx, |show| self.executor_id.and_then(|id| show.executor(&id).cloned()));

        let executor_name =
            executor.as_ref().map(|exec| exec.name().to_string()).unwrap_or_default();

        let is_on = executor.as_ref().is_some_and(|exec| exec.is_on());

        let header = div()
            .flex()
            .items_center()
            .p_1()
            .w_full()
            .bg(if is_on { cx.theme().colors.bg_active } else { cx.theme().colors.bg_primary })
            .text_center()
            .child(div().w_full().child(executor_name));

        let cues = with_show(cx, |show| {
            let sequence = executor.as_ref().and_then(|exec| exec.sequence(show).cloned());

            if let Some(sequence) = &sequence
                && sequence.has_fading_cue()
            {
                window.request_animation_frame();
            }

            let prev_cue = sequence.as_ref().and_then(|seq| seq.previous_cue());
            let current_cue = sequence.as_ref().and_then(|seq| seq.current_cue());
            let next_cue = sequence.as_ref().and_then(|seq| seq.next_cue());

            let render_cue = |sequence: Option<&Sequence>, cue: Option<&Cue>| {
                let content = match (sequence, cue) {
                    (Some(seq), Some(cue)) => {
                        let progress = seq.cue_fade_progress(cue.id());
                        let is_current =
                            seq.current_cue().is_some_and(|current| current.id() == cue.id());
                        let name = format!("{} {}", cue.id().to_string(), cue.name());

                        z_stack([
                            div()
                                .h_full()
                                .w(relative(progress.unwrap_or_default()))
                                .bg(cx.theme().colors.bg_active_bright)
                                .opacity(0.5),
                            div()
                                .h_full()
                                .w(relative(if is_current { 1.0 } else { 0.0 }))
                                .bg(cx.theme().colors.bg_active_bright)
                                .opacity(if progress.is_some() { 0.5 } else { 1.0 }),
                            div()
                                .flex()
                                .items_center()
                                .h_full()
                                .px_1()
                                .whitespace_nowrap()
                                .child(div().w_full().child(name.to_string())),
                        ])
                        .size_full()
                    }
                    _ => div(),
                };
                div().w_full().h_1_3().child(content)
            };

            div()
                .flex()
                .flex_col()
                .child(
                    render_cue(sequence.as_ref(), prev_cue)
                        .border_b_1()
                        .border_color(cx.theme().colors.border),
                )
                .child(
                    render_cue(sequence.as_ref(), current_cue)
                        .border_b_1()
                        .border_color(cx.theme().colors.border)
                        .when(is_on, |e| {
                            e.border_1()
                                .border_color(cx.theme().colors.border_active)
                                .bg(cx.theme().colors.bg_active)
                        }),
                )
                .child(render_cue(sequence.as_ref(), next_cue))
        });

        let render_button = |id, label, cx: &App| {
            div()
                .id(id)
                .h_1_3()
                .w_full()
                .flex()
                .items_center()
                .px_1()
                .bg(cx.theme().colors.bg_primary)
                .cursor_pointer()
                .child(div().w_full().child(label))
                .on_click({
                    let executor_id = self.executor_id;
                    move |_, _, cx| {
                        if let Some(executor_id) = executor_id {
                            exec_cmd_and_log_err(Command::Go { executor_id }, cx);
                        }
                    }
                })
        };

        let controls = div()
            .flex()
            .child(
                div()
                    .w_1_2()
                    .h_full()
                    .bg(cx.theme().colors.bg_primary)
                    .border_r_1()
                    .border_color(cx.theme().colors.border),
            )
            .child(
                div()
                    .w_1_2()
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(
                        render_button("button_3", "On", cx)
                            .border_b_1()
                            .border_color(cx.theme().colors.border),
                    )
                    .child(
                        render_button("button_2", "Off", cx)
                            .border_b_1()
                            .border_color(cx.theme().colors.border),
                    )
                    .child(render_button("button_1", "Go", cx)),
            );

        container(if is_on {
            ContainerStyle {
                background: ContainerStyle::normal(window, cx).background,
                border: cx.theme().colors.border_active,
                text_color: window.text_style().color,
            }
        } else {
            ContainerStyle::normal(window, cx)
        })
        .flex()
        .flex_col()
        .w(CELL_SIZE * 1.0)
        .h(CELL_SIZE * 2.0)
        .child(header.w_full().h(CELL_SIZE * 0.5).border_b_1().border_color(if is_on {
            cx.theme().colors.border_active
        } else {
            cx.theme().colors.border
        }))
        .child(
            cues.w_full().h(CELL_SIZE * 0.75).border_b_1().border_color(cx.theme().colors.border),
        )
        .child(controls.w_full().h(CELL_SIZE * 0.75))
        .into_any_element()
    }
}
