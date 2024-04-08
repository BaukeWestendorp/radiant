use gpui::prelude::FluentBuilder;
use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, Global, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WindowContext,
};
use theme::ActiveTheme;
use ui::button::{Button, ButtonStyle};
use ui::selectable::Selectable;
use ui::text_input::{self, TextInput};

use crate::app::AppState;
use crate::layout::{LayoutView, GRID_SIZE};
use crate::showfile::{Layout, ShowfileManager};

pub mod actions {

    use backstage::Command;
    use gpui::{actions, impl_actions};

    actions!(workspace, [ExecuteCurrentCommand]);

    impl_actions!(workspace, [ExecuteCommand, SetCurrentCommand]);

    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    pub struct ExecuteCommand(pub Command);

    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    pub struct SetCurrentCommand(pub Option<Command>);
}

pub struct Workspace {
    focus_handle: FocusHandle,
    screen: View<Screen>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            focus_handle: cx.focus_handle(),
            screen: Screen::build(cx),
        })
    }

    fn handle_execute_command(
        &mut self,
        cmd: &actions::ExecuteCommand,
        cx: &mut ViewContext<Self>,
    ) {
        if let Err(err) =
            ShowfileManager::update(cx, |showfile, _cx| showfile.show.execute_command(&cmd.0))
        {
            log::error!("Failed to execute command '{}': {err}", cmd.0);
        }
        cx.notify();
    }

    fn handle_set_current_command(
        &mut self,
        action: &actions::SetCurrentCommand,
        cx: &mut ViewContext<Self>,
    ) {
        ShowfileManager::update(cx, |showfile, cx| {
            showfile.show.current_command = action.0.clone();
            cx.notify();
        });
    }

    fn handle_execute_current_command(
        &mut self,
        _action: &actions::ExecuteCurrentCommand,
        cx: &mut ViewContext<Self>,
    ) {
        if let Err(err) = ShowfileManager::update(cx, |showfile, cx| {
            let result = showfile.show.execute_current_command();
            cx.notify();
            result
        }) {
            log::error!(
                "Failed to execute command '{}': {err}",
                ShowfileManager::show(cx)
                    .current_command
                    .map_or("".to_string(), |cmd| cmd.to_string())
            );
        }
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context("Workspace")
            .size_full()
            .text_color(cx.theme().colors().text)
            .font("Zed Sans")
            .on_action(cx.listener(Self::handle_execute_command))
            .on_action(cx.listener(Self::handle_set_current_command))
            .on_action(cx.listener(Self::handle_execute_current_command))
            .track_focus(&self.focus_handle)
            .child(self.screen.clone())
    }
}

pub struct Screen {
    current_layout_view: View<LayoutView>,
    command_line: View<CommandLine>,
}

impl Screen {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let current_layout_view = get_current_layout_view(cx);

        cx.new_view(|cx| {
            cx.observe_global::<ShowfileManager>({
                move |screen: &mut Screen, cx| {
                    screen.current_layout_view = get_current_layout_view(cx);
                    cx.notify();
                }
            })
            .detach();

            Self {
                current_layout_view,
                command_line: CommandLine::build(cx),
            }
        })
    }

    fn render_sidebar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let layouts = ShowfileManager::layouts(cx).clone();

        // FIXME: For now we are showing 10 layouts, but this should be a
        // inplace-scrollable, just like the pool windows.
        let items = (1..=10).map(|id| {
            let layout = layouts.layouts.iter().find(|layout| layout.id == id);
            self.render_layout_item(layout, id, cx).into_any_element()
        });

        div().w_32().children(items)
    }

    fn render_layout_item(
        &self,
        layout: Option<&Layout>,
        id: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let border_color = match ShowfileManager::layouts(cx).selected_layout_id == id {
            true => cx.theme().colors().border_selected,
            false => match layout.is_some() {
                true => cx.theme().colors().border,
                false => cx.theme().colors().border_disabled,
            },
        };

        let display = div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .border_b()
            .border_color(border_color)
            .children(layout.map(|l| l.label.clone()));

        let id_element = div()
            .h_5()
            .px_1()
            .when(layout.is_none(), |this| {
                this.text_color(cx.theme().colors().text_muted)
            })
            .child(id.to_string());

        let content = div()
            .flex()
            .flex_col()
            .h_full()
            .child(display)
            .child(id_element);

        Button::new(ButtonStyle::Primary, id, cx)
            .selected(ShowfileManager::layouts(cx).selected_layout_id == id)
            .border_color(border_color)
            .w_full()
            .h(GRID_SIZE)
            .text_sm()
            .on_click(cx.listener({
                let layout = layout.cloned();
                move |_screen, _event, cx| {
                    if let Some(layout) = &layout {
                        ShowfileManager::update(cx, |showfile, _cx| {
                            showfile.layouts.selected_layout_id = layout.id;
                        });
                        cx.notify();
                    }
                }
            }))
            .child(content)
    }
}

impl Render for Screen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = div()
            .size_full()
            .overflow_hidden()
            .child(self.current_layout_view.clone());

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .size_full()
                    .flex()
                    .child(content)
                    .child(self.render_sidebar(cx)),
            )
            .child(div().w_full().h_10().child(self.command_line.clone()))
    }
}

fn get_current_layout_view(cx: &mut WindowContext) -> View<LayoutView> {
    let current_layout_model = cx.new_model(|cx| {
        // FIXME: Handle nonexistent layout (this should not be possible, but lets
        // softerror on this to be sure).
        ShowfileManager::layouts(cx)
            .layouts
            .iter()
            .find(|layout| layout.id == ShowfileManager::layouts(cx).selected_layout_id)
            .unwrap()
            .clone()
    });

    LayoutView::build(current_layout_model, cx)
}

pub struct CommandLine {
    text_input: View<TextInput>,

    focus_handle: FocusHandle,
}

impl CommandLine {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let focus_handle = cx.focus_handle();
            let text_input =
                cx.new_view(|cx| TextInput::new(None, "Command line", focus_handle.clone(), cx));

            // Update the command input when the current command changes.
            cx.observe_global::<ShowfileManager>(|command_line: &mut Self, cx| {
                command_line.text_input.update(cx, |text_input, cx| {
                    *text_input.text_mut() = ShowfileManager::show(cx)
                        .current_command
                        .map(|cmd| cmd.to_string())
                        .unwrap_or_default();
                    cx.notify();
                });
            })
            .detach();

            cx.subscribe(
                &text_input,
                |cmd_line: &mut CommandLine, _text_input, event, cx| match event {
                    text_input::Event::Submit(input) => {
                        cmd_line.handle_submit_command_input(input, cx)
                    }
                },
            )
            .detach();

            cx.on_focus_in(&focus_handle, |_focus_handle, cx| {
                AppState::update(cx, |app_state, _cx| app_state.use_command_shortcuts = false)
            })
            .detach();

            cx.on_blur(&focus_handle, |_focus_handle, cx| {
                AppState::update(cx, |app_state, _cx| app_state.use_command_shortcuts = true)
            })
            .detach();

            Self {
                text_input,
                focus_handle,
            }
        })
    }

    fn handle_submit_command_input(&mut self, input: &str, cx: &mut WindowContext) {
        if input.is_empty() {
            return;
        }

        self.text_input.update(cx, |text_input, cx| {
            text_input.clear(cx);
            ShowfileManager::update(cx, |showfile, cx| {
                if let Err(err) = showfile.show.execute_command_str(input) {
                    log::error!("Failed to execute command: {}", err.to_string())
                } else {
                    cx.notify();
                }
            })
        })
    }
}

impl FocusableView for CommandLine {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CommandLine {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .border_t()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().element_background)
            .flex()
            .flex_shrink()
            .items_center()
            .px_3()
            .child(self.text_input.clone())
            .track_focus(&self.focus_handle)
    }
}
