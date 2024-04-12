use backstage::{Command, Object, Preset};
use gpui::{
    prelude::FluentBuilder, IntoElement, Model, ParentElement, Styled, ViewContext, WindowContext,
};
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::{ShowfileManager, Window};
use theme::ActiveTheme;

pub struct BeamPoolWindowDelegate {
    window: Model<Window>,
}

impl BeamPoolWindowDelegate {
    pub fn new(window: Model<Window>) -> Self {
        Self { window }
    }
}

impl PoolWindowDelegate for BeamPoolWindowDelegate {
    fn label(&self) -> String {
        "Beam".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        let Some(beam_preset) = ShowfileManager::show(cx).beam_preset(id) else {
            return None;
        };

        let is_in_programmer = ShowfileManager::show(cx)
            .attribute_values_in_programmer(beam_preset.attribute_values());

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
                .child(beam_preset.label().to_string()),
        )
    }

    fn handle_click_item(&mut self, id: usize, cx: &mut ViewContext<WindowView<Self>>) {
        ShowfileManager::update(cx, |showfile, _cx| {
            if let Err(err) = showfile
                .show
                .execute_command(&Command::Select(Some(Object::PresetBeam(id))))
            {
                log::error!("Failed to select beam preset {id}: {err}");
            }
        });
        cx.notify();
    }
}
