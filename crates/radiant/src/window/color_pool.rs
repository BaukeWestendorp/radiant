use backstage::{Command, Object, Preset};
use gpui::prelude::FluentBuilder;
use gpui::{div, IntoElement, Model, ParentElement, Rgba, Styled, ViewContext, WindowContext};
use theme::ActiveTheme;
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::color::Color;
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
        "Color".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        let Some(color_preset) = ShowfileManager::show(cx).color_preset(id) else {
            return None;
        };

        let color_rgba = Color::from_attribute_values(color_preset.attribute_values()).ok();

        Some(
            Button::new(ButtonStyle::Secondary, id, cx)
                .size_full()
                .flex()
                .flex_col()
                .child(
                    div()
                        .h_full()
                        .flex()
                        .justify_center()
                        .items_center()
                        .child(color_preset.label().to_string()),
                )
                .when_some(color_rgba, |this, color| {
                    this.child(
                        div()
                            .h_5()
                            .w_full()
                            .border_t()
                            .border_color(Rgba::from(color.clone().darkened(0.2)))
                            .bg(Rgba::from(color)),
                    )
                }),
        )
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
