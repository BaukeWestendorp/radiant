use gpui::*;
use show::{AssetPool, Group, GroupId};
use ui::{Button, ButtonKind};

use super::AssetSelectorEvent;

pub struct GroupSelector {
    id: ElementId,
    pool: Model<AssetPool<Group>>,
    selected_group: Option<GroupId>,
}

impl GroupSelector {
    pub fn new(id: impl Into<ElementId>, pool: Model<AssetPool<Group>>) -> Self {
        Self {
            id: id.into(),
            pool,
            selected_group: None,
        }
    }

    pub fn selected_group<'a>(&self, cx: &'a AppContext) -> Option<&'a Group> {
        self.selected_group
            .and_then(|id| self.pool.read(cx).get(&id))
    }
}

impl Render for GroupSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // TODO: We should use this label, but the button element does not support passing elements as labels yet.
        // let display_label = match self.selected_group(cx) {
        //     Some(group) => div().child(group.label.clone()),
        //     None => div()
        //         .italic()
        //         .text_color(cx.theme().text_muted)
        //         .child("None"),
        // };

        let label = match self.selected_group(cx) {
            Some(group) => group.label.clone(),
            None => "None".to_string(),
        };

        Button::new(ButtonKind::Primary, label, self.id.clone())
            .on_click(cx.listener(|this, _, cx| todo!("handle click")))
    }
}

impl EventEmitter<AssetSelectorEvent<GroupId>> for GroupSelector {}
