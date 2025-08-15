use gpui::prelude::*;
use gpui::{ReadGlobal, Window, div};

use crate::panel::window::{WindowPanel, WindowPanelDelegate};
use crate::state::AppState;

pub struct CommandLinePanel {
    prompt: String,
}

impl CommandLinePanel {
    pub fn new(cx: &mut Context<WindowPanel<Self>>) -> Self {
        let cb = AppState::global(cx).command_builder.clone();
        cx.observe(&cb, |panel, cb, cx| {
            panel.delegate.prompt = cb.read(cx).to_string();
            cx.notify();
        })
        .detach();

        Self { prompt: "".to_string() }
    }
}

impl WindowPanelDelegate for CommandLinePanel {
    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowPanel<Self>>,
    ) -> impl IntoElement {
        let output = div().size_full().bg(gpui::red());
        let prompt = div().size_full().child(self.prompt.clone());

        div().size_full().flex().flex_col().child(output).child(prompt)
    }
}
