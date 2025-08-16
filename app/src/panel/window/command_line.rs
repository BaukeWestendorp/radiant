use gpui::prelude::*;
use gpui::{Entity, ReadGlobal, Window, div};
use ui::{ActiveTheme, Field};

use crate::panel::window::{WindowPanel, WindowPanelDelegate};
use crate::state::AppState;

pub struct CommandLinePanel {
    prompt: Entity<Field<String>>,
}

impl CommandLinePanel {
    pub fn new(window: &mut Window, cx: &mut Context<WindowPanel<Self>>) -> Self {
        let cb = AppState::global(cx).command_builder.clone();
        cx.observe(&cb, |panel, cb, cx| {
            panel.delegate.prompt.update(cx, |prompt, cx| {
                let value = cb.read(cx).to_string();
                prompt.set_value(&value, cx);
            });
            cx.notify();
        })
        .detach();

        Self { prompt: cx.new(|cx| Field::new("prompt", cx.focus_handle(), window, cx)) }
    }
}

impl WindowPanelDelegate for CommandLinePanel {
    fn render_content(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<WindowPanel<Self>>,
    ) -> impl IntoElement {
        let lines = AppState::global(cx)
            .engine()
            .command_history()
            .iter()
            .map(|command| command.to_string());
        let history = div().id("command_history").size_full().overflow_scroll().children(lines);

        let prompt = div()
            .p_1()
            .border_t_1()
            .border_color(cx.theme().colors.border)
            .w_full()
            .child(self.prompt.clone());

        div().size_full().flex().flex_col().child(history).child(prompt)
    }
}
