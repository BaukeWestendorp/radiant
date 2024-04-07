use backstage::command::{Command, Object};
use backstage::show::{Cue, Executor, ExecutorButton, ExecutorButtonAction, Sequence};
use gpui::prelude::FluentBuilder;
use gpui::{
    div, uniform_list, IntoElement, MouseDownEvent, MouseUpEvent, ParentElement, Render,
    SharedString, Styled, View, ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;
use ui::button::{Button, ButtonStyle};
use ui::container::Container;

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
        cx.new_view(|cx| Self {
            executor_views: get_executor_views(cx),
        })
    }
}

fn get_executor_views(cx: &mut WindowContext) -> Vec<View<ExecutorView>> {
    ShowfileManager::show(cx)
        .executors()
        .clone()
        .iter()
        .map(|executor| ExecutorView::build(executor.clone(), cx))
        .collect::<Vec<_>>()
}

impl Render for ExecutorsWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .children(self.executor_views.clone())
    }
}

pub struct ExecutorView {
    info: View<ExecutorInfo>,
    button_1: View<ExecutorButtonView>,
    button_2: View<ExecutorButtonView>,
    button_3: View<ExecutorButtonView>,
}

impl ExecutorView {
    pub fn build(executor: Executor, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            info: ExecutorInfo::build(executor.clone(), cx),
            button_1: ExecutorButtonView::build(executor.id, executor.button_1, cx),
            button_2: ExecutorButtonView::build(executor.id, executor.button_2, cx),
            button_3: ExecutorButtonView::build(executor.id, executor.button_3, cx),
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
    executor: Executor,
}

impl ExecutorInfo {
    pub fn build(executor: Executor, cx: &mut WindowContext) -> View<Self> {
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
                cues[range]
                    .iter()
                    .enumerate()
                    .map(|(ix, cue)| {
                        let active = this.executor.current_index.get() == Some(ix);
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
            .sequence
            .and_then(|id| ShowfileManager::show(cx).sequence(id).cloned());

        Container::new(cx)
            .size(GRID_SIZE)
            .text_xs()
            .child(self.render_header(sequence.as_ref(), cx))
            .child(self.render_cues(sequence.as_ref(), cx))
    }
}

pub struct ExecutorButtonView {
    executor_id: usize,
    button: ExecutorButton,
}

impl ExecutorButtonView {
    pub fn build(executor_id: usize, button: ExecutorButton, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {
            executor_id,
            button,
        })
    }

    pub fn handle_press(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        match self.button.action {
            ExecutorButtonAction::Go => {
                ShowfileManager::update(cx, |showfile, _cx| {
                    if let Err(err) = showfile
                        .show
                        .execute_command(&Command::Go(Some(Object::Executor(self.executor_id))))
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
                        .execute_command(&Command::Top(Some(Object::Executor(self.executor_id))))
                    {
                        log::error!("Failed to execute 'top' command: {}", err.to_string());
                    }
                });

                cx.notify();
            }
            ExecutorButtonAction::Flash => {
                ShowfileManager::update(cx, |showfile, _cx| {
                    if let Some(executor) = showfile.show.executor_mut(self.executor_id) {
                        executor.flash = true;
                    }
                });

                cx.notify();
            }
        }
    }

    pub fn handle_release(&mut self, _event: &MouseUpEvent, cx: &mut ViewContext<Self>) {
        if self.button.action == ExecutorButtonAction::Flash {
            ShowfileManager::update(cx, |showfile, _cx| {
                if let Some(executor) = showfile.show.executor_mut(self.executor_id) {
                    executor.flash = false;
                }
            });
            cx.notify();
        }
    }
}

impl Render for ExecutorButtonView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Button::new(ButtonStyle::Primary, self.executor_id, cx)
            .w(GRID_SIZE)
            .h(GRID_SIZE / 2.0)
            .flex()
            .justify_center()
            .items_center()
            .child(self.button.action.to_string())
        // .on_click_down(cx.listener(Self::handle_click))
        // .on_click_up(cx.listener(Self::handle_release))
    }
}
