use backstage::command::{Command, Instruction, Object};
use backstage::show::{Cue, Executor, ExecutorButton, ExecutorButtonAction, Sequence, Show};
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, uniform_list, InteractiveElement, IntoElement, Model, MouseButton, MouseDownEvent,
    MouseUpEvent, ParentElement, Render, Styled, View, ViewContext, VisualContext, WindowContext,
};

use super::{WindowDelegate, WindowView};
use crate::theme::ActiveTheme;
use crate::workspace::layout::window_grid::{grid_div, GridSize, GRID_CELL_SIZE};

pub struct ExecutorsWindowDelegate {
    executors_window: View<ExecutorsWindow>,
}

impl ExecutorsWindowDelegate {
    pub fn new(show: Model<Show>, cx: &mut WindowContext) -> Self {
        Self {
            executors_window: ExecutorsWindow::build(show.clone(), cx),
        }
    }
}

impl WindowDelegate for ExecutorsWindowDelegate {
    fn title(&self) -> String {
        "Executors".to_string()
    }

    fn render_content(&self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().child(self.executors_window.clone())
    }
}

pub struct ExecutorsWindow {
    executor_views: Vec<View<ExecutorView>>,
}

impl ExecutorsWindow {
    pub fn build(show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&show, |this: &mut Self, show, cx| {
                this.executor_views = get_executor_views(show, cx);
                cx.notify();
            })
            .detach();

            Self {
                executor_views: get_executor_views(show.clone(), cx),
            }
        })
    }
}

fn get_executor_views(show: Model<Show>, cx: &mut WindowContext) -> Vec<View<ExecutorView>> {
    show.read(cx)
        .executors()
        .clone()
        .iter()
        .map(|executor| ExecutorView::build(executor.clone(), show.clone(), cx))
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
    pub fn build(executor: Executor, show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            info: ExecutorInfo::build(executor.clone(), show.clone(), cx),
            button_1: ExecutorButtonView::build(executor.id, executor.button_1, show.clone(), cx),
            button_2: ExecutorButtonView::build(executor.id, executor.button_2, show.clone(), cx),
            button_3: ExecutorButtonView::build(executor.id, executor.button_3, show.clone(), cx),
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
            .bg(cx.theme().colors().background_tertriary)
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
            .rounded_md()
            .child(cue.label.clone())
            .when(active, |this| this.bg(cx.theme().colors().element_selected))
    }
}

impl Render for ExecutorInfo {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let sequence = self
            .executor
            .sequence
            .and_then(|id| self.show.read(cx).sequence(id).cloned());

        grid_div(GridSize::new(1, 1), None)
            .bg(cx.theme().colors().background)
            .border()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(self.render_header(sequence.as_ref(), cx))
            .child(self.render_cues(sequence.as_ref(), cx))
    }
}

pub struct ExecutorButtonView {
    executor_id: usize,
    button: ExecutorButton,

    show: Model<Show>,
}

impl ExecutorButtonView {
    pub fn build(
        executor_id: usize,
        button: ExecutorButton,
        show: Model<Show>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self {
            executor_id,
            button,
            show,
        })
    }

    pub fn handle_click(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        match self.button.action {
            ExecutorButtonAction::Go => self.show.update(cx, |show, cx| {
                if let Err(err) = show.execute_command(&Command::new([
                    Instruction::Select(Object::Executor(self.executor_id)),
                    Instruction::Go,
                ])) {
                    log::error!("Failed to execute Go command: {}", err.to_string());
                }
                cx.notify();
            }),
            ExecutorButtonAction::Top => self.show.update(cx, |show, cx| {
                if let Err(err) = show.execute_command(&Command::new([
                    Instruction::Select(Object::Executor(self.executor_id)),
                    Instruction::Top,
                ])) {
                    log::error!("Failed to execute Top command: {}", err.to_string());
                }
                cx.notify();
            }),
            ExecutorButtonAction::Flash => {
                self.show.update(cx, |show, cx| {
                    if let Some(executor) = show.executor_mut(self.executor_id) {
                        executor.flash = true;
                        cx.notify();
                    }
                });
            }
        }
    }

    pub fn handle_release(&mut self, _event: &MouseUpEvent, cx: &mut ViewContext<Self>) {
        match self.button.action {
            ExecutorButtonAction::Flash => {
                self.show.update(cx, |show, cx| {
                    if let Some(executor) = show.executor_mut(self.executor_id) {
                        executor.flash = false;
                        cx.notify();
                    }
                });
            }
            _ => {}
        }
    }
}

impl Render for ExecutorButtonView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let executor = self.show.read(cx).executor(self.executor_id);

        div()
            .w(px(GRID_CELL_SIZE as f32))
            .h(px(GRID_CELL_SIZE as f32 / 2.0))
            .bg(cx.theme().colors().element_background)
            .border()
            .border_color(
                match self.button.action == ExecutorButtonAction::Flash
                    && executor.is_some_and(|e| e.flash)
                {
                    true => cx.theme().colors().element_selected,
                    false => cx.theme().colors().border,
                },
            )
            .rounded_md()
            .flex()
            .justify_center()
            .items_center()
            .hover(|this| this.bg(cx.theme().colors().element_hover))
            .cursor_pointer()
            .child(self.button.action.to_string())
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_click))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_release))
    }
}
