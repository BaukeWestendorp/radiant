use backstage::{Command, Object, Preset};
use gpui::{IntoElement, Model, ParentElement, Styled, ViewContext, WindowContext};
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::{ShowfileManager, Window};

pub struct DimmerPoolWindowDelegate {
    window: Model<Window>,
}

impl DimmerPoolWindowDelegate {
    pub fn new(window: Model<Window>) -> Self {
        Self { window }
    }
}

impl PoolWindowDelegate for DimmerPoolWindowDelegate {
    fn label(&self) -> String {
        "Dimmer".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        ShowfileManager::show(cx).dimmer_preset(id).map(|dimmer| {
            Button::new(ButtonStyle::Secondary, id, cx)
                .size_full()
                .flex()
                .justify_center()
                .items_center()
                .child(dimmer.label().to_string())
        })
    }

    fn handle_click_item(&mut self, id: usize, cx: &mut ViewContext<WindowView<Self>>) {
        ShowfileManager::update(cx, |showfile, _cx| {
            if let Err(err) = showfile
                .show
                .execute_command(&Command::Select(Some(Object::PresetDimmer(id))))
            {
                log::error!("Failed to select dimmer preset {id}: {err}");
            }
        });
        cx.notify();
    }
}
