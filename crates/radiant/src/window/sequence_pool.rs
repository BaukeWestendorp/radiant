use backstage::{Command, Object};
use gpui::{div, px, IntoElement, Model, ParentElement, Styled, ViewContext, WindowContext};
use ui::button::{Button, ButtonStyle};

use super::pool::PoolWindowDelegate;
use super::WindowView;
use crate::showfile::{ShowfileManager, Window};

pub struct SequencePoolWindowDelegate {
    window: Model<Window>,
}

impl SequencePoolWindowDelegate {
    pub fn new(window: Model<Window>) -> Self {
        Self { window }
    }
}

impl PoolWindowDelegate for SequencePoolWindowDelegate {
    fn label(&self) -> String {
        "Sequence".to_string()
    }

    fn window(&self) -> &Model<Window> {
        &self.window
    }

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement> {
        if let Some(sequence) = ShowfileManager::show(cx).sequence(id) {
            Some(
                Button::new(ButtonStyle::Secondary, id, cx)
                    .size_full()
                    .flex()
                    .justify_center()
                    .line_height(px(12.0))
                    .items_center()
                    .p_2()
                    .child(div().w_full().text_xs().child(sequence.label.clone())),
            )
        } else {
            None
        }
    }

    fn handle_click_item(&mut self, id: usize, cx: &mut ViewContext<WindowView<Self>>) {
        ShowfileManager::update(cx, |showfile, cx| {
            if let Some(Command::Store(object)) = &mut showfile.show.current_command {
                *object = Some(Object::Sequence(id));
                if let Err(err) = showfile.show.execute_current_command() {
                    log::error!("Failed to store sequence {id}: {err}");
                }
            } else {
                if let Err(err) = showfile
                    .show
                    .execute_command(&Command::Select(Some(Object::Sequence(id))))
                {
                    log::error!("Failed to select sequence {id}: {err}");
                }
            }

            cx.notify();
        });
    }
}
