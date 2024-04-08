use backstage::{Command, Object, Preset};
use gpui::{IntoElement, Model, ParentElement, Styled, ViewContext, WindowContext};
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::{ShowfileManager, Window};

pub struct FocusPoolWindowDelegate {
    window: Model<Window>,
}

impl FocusPoolWindowDelegate {
    pub fn new(window: Model<Window>) -> Self {
        Self { window }
    }
}

impl PoolWindowDelegate for FocusPoolWindowDelegate {
    fn label(&self) -> String {
        "Focus".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        ShowfileManager::show(cx).focus_preset(id).map(|focus| {
            Button::new(ButtonStyle::Secondary, id, cx)
                .size_full()
                .flex()
                .justify_center()
                .items_center()
                .child(focus.label().to_string())
        })
    }

    fn handle_click_item(&mut self, id: usize, cx: &mut ViewContext<WindowView<Self>>) {
        ShowfileManager::update(cx, |showfile, _cx| {
            if let Err(err) = showfile
                .show
                .execute_command(&Command::Select(Some(Object::PresetFocus(id))))
            {
                log::error!("Failed to select focus preset {id}: {err}");
            }
        });
        cx.notify();
    }
}
