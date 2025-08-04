use gpui::prelude::*;
use gpui::{App, Entity, UpdateGlobal, Window, div, relative};
use radiant::engine::Command;
use radiant::show::{Cue, Executor, Object, ObjectId, Sequence};
use ui::utils::z_stack;
use ui::{ActiveTheme, ContainerStyle, container};

use crate::app::{AppState, with_show};
use crate::main_window::CELL_SIZE;

pub struct ExecutorsPanel {
    executors: Vec<Entity<ExecutorView>>,
}

impl ExecutorsPanel {
    pub fn new(columns: u32, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            executors: (0..columns)
                .into_iter()
                .map(|ix| {
                    cx.new(|_| {
                        let id = ObjectId::new(ix);
                        ExecutorView::new(id)
                    })
                })
                .collect(),
        }
    }
}

impl Render for ExecutorsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().flex().children(self.executors.clone())
    }
}

struct ExecutorView {
    executor_id: ObjectId<Executor>,
}

impl ExecutorView {
    pub fn new(executor_id: ObjectId<Executor>) -> Self {
        Self { executor_id }
    }
}

impl Render for ExecutorView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let executor = with_show(cx, |show| show.executors.get(self.executor_id).cloned());

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
                        AppState::update_global(cx, {
                            move |show, _cx| {
                                show.engine
                                    .exec(Command::Go { executor: executor_id })
                                    .map_err(|err| log::error!("executor command error: {:?}", err))
                                    .ok();
                            }
                        })
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
