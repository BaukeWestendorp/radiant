use gpui::*;
use show::{AssetPool, Group, GroupId};
use ui::{ActiveTheme, Selector, SelectorDelegate};

pub struct GroupSelectorDelegate {
    pool: Model<AssetPool<Group>>,
}

impl GroupSelectorDelegate {
    pub fn new(pool: Model<AssetPool<Group>>) -> Self {
        Self { pool }
    }
}

impl SelectorDelegate for GroupSelectorDelegate {
    type Item = GroupId;

    fn render_display_label(
        &self,
        item: Option<&Self::Item>,
        cx: &mut ViewContext<Selector<Self>>,
    ) -> impl IntoElement {
        match item {
            Some(id) => {
                let group = self.pool.read(cx).get(id).unwrap();
                div().child(group.label.clone())
            }
            None => div()
                .italic()
                .text_color(cx.theme().text_muted)
                .child("None"),
        }
        .px_1()
        .w_full()
        .overflow_hidden()
        .text_ellipsis()
    }

    fn len(&self, cx: &ViewContext<Selector<Self>>) -> usize {
        self.pool.read(cx).len()
    }

    fn items(&self, cx: &ViewContext<Selector<Self>>) -> Vec<Self::Item> {
        self.pool.read(cx).ids().copied().collect()
    }

    fn render_item(
        &self,
        item: &Self::Item,
        cx: &mut ViewContext<Selector<Self>>,
    ) -> impl IntoElement {
        let group = self.pool.read(cx).get(item).unwrap();
        div().child(group.label.clone())
    }
}
