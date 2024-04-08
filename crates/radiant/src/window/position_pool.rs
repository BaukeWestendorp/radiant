use backstage::{Command, Object, Preset};
use gpui::{IntoElement, Model, ParentElement, Styled, ViewContext, WindowContext};
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::{ShowfileManager, Window};

pub struct PositionPoolWindowDelegate {
    window: Model<Window>,
}

impl PositionPoolWindowDelegate {
    pub fn new(window: Model<Window>) -> Self {
        Self { window }
    }
}

impl PoolWindowDelegate for PositionPoolWindowDelegate {
    fn label(&self) -> String {
        "Position".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        ShowfileManager::show(cx)
            .position_preset(id)
            .map(|position| {
                Button::new(ButtonStyle::Secondary, id, cx)
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(position.label().to_string())
            })
    }

    fn handle_click_item(&mut self, id: usize, cx: &mut ViewContext<WindowView<Self>>) {
        ShowfileManager::update(cx, |showfile, _cx| {
            if let Err(err) = showfile
                .show
                .execute_command(&Command::Select(Some(Object::PresetPosition(id))))
            {
                log::error!("Failed to select position preset {id}: {err}");
            }
        });
        cx.notify();
    }
}
