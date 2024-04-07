use backstage::{Command, Object, Preset};
use gpui::{IntoElement, Model, ParentElement, Styled, ViewContext, WindowContext};
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::{ShowfileManager, Window};

pub struct ColorPoolWindowDelegate {
    window: Model<Window>,
}

impl ColorPoolWindowDelegate {
    pub fn new(window: Model<Window>) -> Self {
        Self { window }
    }
}

impl PoolWindowDelegate for ColorPoolWindowDelegate {
    fn label(&self) -> String {
        "Colors".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        if let Some(color) = ShowfileManager::show(cx).color_preset(id) {
            Some(
                Button::new(ButtonStyle::Secondary, id, cx)
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(color.label().to_string()),
            )
        } else {
            None
        }
    }

    fn handle_click_item(&mut self, id: usize, cx: &mut ViewContext<WindowView<Self>>) {
        ShowfileManager::update(cx, |showfile, _cx| {
            if let Err(err) = showfile
                .show
                .execute_command(&Command::Select(Some(Object::PresetColor(id))))
            {
                log::error!("Failed to select color preset {id}: {err}");
            }
        });
        cx.notify();
    }
}
