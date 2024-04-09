use backstage::command::{Command, Object};
use backstage::show::{Cue, Executor, ExecutorButton, ExecutorButtonAction, Sequence};
use gpui::prelude::FluentBuilder;
use gpui::{
    div, uniform_list, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, MouseUpEvent,
    ParentElement, Render, SharedString, Styled, View, ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;
use ui::button::{Button, ButtonStyle};
use ui::container::Container;
use ui::disableable::Disableable;

use super::{WindowDelegate, WindowView};
use crate::layout::GRID_SIZE;
use crate::showfile::ShowfileManager;

pub struct ExecutorsWindowDelegate {
    executors_window: View<ExecutorsWindow>,
}

impl ExecutorsWindowDelegate {
    pub fn new(cx: &mut WindowContext) -> Self {
        Self {
            executors_window: ExecutorsWindow::build(cx),
        }
    }
}

impl WindowDelegate for ExecutorsWindowDelegate {
    fn title(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some("Executors".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().child(self.executors_window.clone())
    }
}

pub struct ExecutorsWindow {
    executor_views: Vec<View<ExecutorView>>,
}

impl ExecutorsWindow {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe_global::<ShowfileManager>(|this: &mut Self, cx| {
                this.executor_views = get_executor_views(cx);
                cx.notify();
            })
            .detach();

            Self {
                executor_views: get_executor_views(cx),
            }
        })
    }
}

fn get_executor_views(cx: &mut WindowContext) -> Vec<View<ExecutorView>> {
    // FIXME: For now we just show 20 executors, but this amount should be
    // calculated based on the width.
    (1..=20)
        .map(|id| {
            let executor = ShowfileManager::show(cx).executor(id);
            ExecutorView::build(id, executor.cloned(), cx)
        })
        .collect::<Vec<_>>()
}

impl Render for ExecutorsWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .children(
                self.executor_views
                    .clone()
                    .into_iter()
                    .map(|executor_view| {
                        div()
                            .on_mouse_down(MouseButton::Left, {
                                let executor_view = executor_view.clone();
                                move |_event, cx| {
                                    ShowfileManager::update(cx, |showfile, cx| {
                                        if let Some(Command::Store(object)) =
                                            &mut showfile.show.current_command
                                        {
                                            *object = Some(Object::Executor(Some(
                                                executor_view.read(cx).id,
                                            )));
                                        }
                                        if let Err(err) = showfile.show.execute_current_command() {
                                            log::error!("Failed to execute current command: {err}");
                                        }
                                    })
                                }
                            })
                            .child(executor_view.clone())
                    }),
            )
    }
}

pub struct ExecutorView {
    id: usize,
    info: View<ExecutorInfo>,
    button_1: View<ExecutorButtonView>,
    button_2: View<ExecutorButtonView>,
    button_3: View<ExecutorButtonView>,
}

impl ExecutorView {
    pub fn build(id: usize, executor: Option<Executor>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            id,
            info: ExecutorInfo::build(executor.clone(), cx),
            button_1: ExecutorButtonView::build(executor.clone(), ExecutorButtonKind::First, cx),
            button_2: ExecutorButtonView::build(executor.clone(), ExecutorButtonKind::Second, cx),
            button_3: ExecutorButtonView::build(executor.clone(), ExecutorButtonKind::Third, cx),
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
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(self.button_1.clone())
                    .child(self.button_2.clone())
                    .child(self.button_3.clone()),
            )
    }
}

pub struct ExecutorInfo {
    executor: Option<Executor>,
}

impl ExecutorInfo {
    pub fn build(executor: Option<Executor>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { executor })
    }

    fn render_header(
        &self,
        sequence: Option<&Sequence>,
        cx: &mut WindowContext,
    ) -> impl IntoElement {
        div()
            .bg(cx.theme().colors().element_background_secondary)
            .h_5()
            .overflow_hidden()
            .border_b()
            .border_color(cx.theme().colors().border)
            .px_1()
            .when(sequence.is_none(), |this| {
                this.text_color(cx.theme().colors().text_disabled)
            })
            .child(
                sequence
                    .map(|s| s.label.clone())
                    .unwrap_or("Empty".to_string()),
            )
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
                let Some(executor) = this.executor.clone() else {
                    return vec![];
                };

                cues[range]
                    .iter()
                    .enumerate()
                    .map(|(ix, cue)| {
                        let active = executor.current_index.get() == Some(ix);
                        this.render_cue(cue, active, cx)
                    })
                    .collect()
            },
        )
    }

    fn render_cue(&self, cue: &Cue, active: bool, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .px_1()
            .border_b()
            .border_color(cx.theme().colors().border)
            .when(active, |this| {
                this.bg(cx.theme().colors().element_background_selected)
            })
            .child(cue.label.clone())
    }
}

impl Render for ExecutorInfo {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let sequence = self
            .executor
            .as_ref()
            .and_then(|e| e.sequence)
            .and_then(|id| ShowfileManager::show(cx).sequence(id).cloned());

        Container::new(cx)
            .size(GRID_SIZE)
            .text_xs()
            .child(self.render_header(sequence.as_ref(), cx))
            .child(self.render_cues(sequence.as_ref(), cx))
    }
}

pub struct ExecutorButtonView {
    executor: Option<Executor>,
    button: Option<ExecutorButton>,
}

impl ExecutorButtonView {
    pub fn build(
        executor: Option<Executor>,
        kind: ExecutorButtonKind,
        cx: &mut WindowContext,
    ) -> View<Self> {
        let button = executor.as_ref().map(|e| match kind {
            ExecutorButtonKind::First => e.button_1.clone(),
            ExecutorButtonKind::Second => e.button_2.clone(),
            ExecutorButtonKind::Third => e.button_3.clone(),
        });

        cx.new_view(|_cx| Self { executor, button })
    }

    pub fn handle_press(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        let Some(button) = self.button.clone() else {
            return;
        };

        let Some(executor_id) = self.executor.as_ref().map(|e| e.id) else {
            return;
        };

        match button.action {
            ExecutorButtonAction::Go => {
                ShowfileManager::update(cx, |showfile, _cx| {
                    if let Err(err) = showfile
                        .show
                        .execute_command(&Command::Go(Some(Object::Executor(Some(executor_id)))))
                    {
                        log::error!("Failed to execute 'go' command: {}", err.to_string());
                    }
                });

                cx.notify();
            }
            ExecutorButtonAction::Top => {
                ShowfileManager::update(cx, |showfile, _cx| {
                    if let Err(err) = showfile
                        .show
                        .execute_command(&Command::Top(Some(Object::Executor(Some(executor_id)))))
                    {
                        log::error!("Failed to execute 'top' command: {}", err.to_string());
                    }
                });

                cx.notify();
            }
            ExecutorButtonAction::Flash => {
                ShowfileManager::update(cx, |showfile, cx| {
                    if let Some(executor) = showfile.show.executor_mut(executor_id) {
                        executor.flash = true;
                        cx.notify();
                    }
                });
            }
        }
    }

    pub fn handle_release(&mut self, _event: &MouseUpEvent, cx: &mut ViewContext<Self>) {
        let Some(button) = self.button.clone() else {
            return;
        };

        let Some(executor_id) = self.executor.as_ref().map(|e| e.id) else {
            return;
        };

        if button.action == ExecutorButtonAction::Flash {
            ShowfileManager::update(cx, |showfile, cx| {
                if let Some(executor) = showfile.show.executor_mut(executor_id) {
                    executor.flash = false;
                    cx.notify();
                }
            });
        }
    }
}

impl Render for ExecutorButtonView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Button::new(ButtonStyle::Primary, "executor_button", cx)
            .w(GRID_SIZE)
            .h(GRID_SIZE / 2.0)
            .flex()
            .justify_center()
            .items_center()
            .disabled(self.executor.is_none())
            .when_some(self.button.clone(), |this, button| {
                this.child(button.action.to_string())
            })
            .on_press(cx.listener(Self::handle_press))
            .on_release(cx.listener(Self::handle_release))
    }
}

pub enum ExecutorButtonKind {
    First,
    Second,
    Third,
}
