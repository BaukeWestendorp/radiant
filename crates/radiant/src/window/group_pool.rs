use backstage::{Command, Object};
use gpui::prelude::FluentBuilder;
use gpui::{IntoElement, Model, ParentElement, Styled, ViewContext, WindowContext};
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
        "Group".to_string()
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
                        this.border()
                            .border_color(cx.theme().colors().pool_item_all_selected)
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

    fn handle_click_item(&mut self, id: usize, cx: &mut ViewContext<WindowView<Self>>) {
        ShowfileManager::update(cx, |showfile, cx| {
            if let Some(Command::Store(object)) = &mut showfile.show.current_command {
                *object = Some(Object::Group(Some(id)));
                if let Err(err) = showfile.show.execute_current_command() {
                    log::error!("Failed to store group {id}: {err}");
                }
            } else {
                if let Err(err) = showfile
                    .show
                    .execute_command(&Command::Select(Some(Object::Group(Some(id)))))
                {
                    log::error!("Failed to select group {id}: {err}");
                }
            }

            cx.notify();
        });
    }
}
