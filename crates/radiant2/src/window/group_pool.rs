use backstage::{Command, Object};
use gpui::prelude::FluentBuilder;
use gpui::{IntoElement, Model, ParentElement, Styled, WindowContext};
use theme::ActiveTheme;
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::{ShowfileManager, Window};

pub struct GroupPoolWindowDelegate {
    window: Model<Window>,
}

impl GroupPoolWindowDelegate {
    pub fn new(window: Model<Window>) -> Self {
        Self { window }
    }
}

impl PoolWindowDelegate for GroupPoolWindowDelegate {
    fn label(&self) -> String {
        "Groups".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        if let Some(group) = ShowfileManager::show(cx).group(id) {
            let is_selected = ShowfileManager::show(cx).are_fixtures_selected(&group.fixtures);

            Some(
                Button::new(ButtonStyle::Secondary, id, cx)
                    .when(is_selected, |this| {
                        this.border_color(cx.theme().colors().pool_item_all_selected)
                    })
                    .size_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(group.label.clone()),
            )
        } else {
            None
        }
    }

    fn handle_click_item(&mut self, id: usize, cx: &mut gpui::ViewContext<WindowView<Self>>) {
        ShowfileManager::update(cx, |showfile, _cx| {
            if let Err(err) = showfile
                .show
                .execute_command(&Command::Select(Some(Object::Group(id))))
            {
                log::error!("Failed to select group {id}: {err}");
            }
        });
        cx.notify();
    }
}
