use backstage::{Command, Object, Preset};
use gpui::{
    prelude::FluentBuilder, IntoElement, Model, ParentElement, Styled, ViewContext, WindowContext,
};
use theme::ActiveTheme;
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
        let Some(position_preset) = ShowfileManager::show(cx).position_preset(id) else {
            return None;
        };

        let is_in_programmer = ShowfileManager::show(cx)
            .attribute_values_in_programmer(position_preset.attribute_values());

        Some(
            Button::new(ButtonStyle::Secondary, id, cx)
                .size_full()
                .flex()
                .justify_center()
                .items_center()
                .when(is_in_programmer, |this| {
                    this.border()
                        .border_color(cx.theme().colors().programmer_change)
                })
                .child(position_preset.label().to_string()),
        )
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
