use gpui::*;
use show::{AssetPool, EffectGraph, Group};
use ui::{ActiveTheme, Selector, SelectorDelegate};

pub type GroupSelector = Selector<AssetSelectorDelegate<Group>>;
pub type EffectGraphSelector = Selector<AssetSelectorDelegate<EffectGraph>>;

pub struct AssetSelectorDelegate<A: show::Asset> {
    pool: Model<AssetPool<A>>,
}

impl<A: show::Asset> AssetSelectorDelegate<A> {
    pub fn new(pool: Model<AssetPool<A>>) -> Self {
        Self { pool }
    }
}

impl<A: show::Asset + 'static> SelectorDelegate for AssetSelectorDelegate<A> {
    type Item = A::Id;

    fn render_display_label(
        &self,
        item: Option<&Self::Item>,
        cx: &ViewContext<Selector<Self>>,
    ) -> impl IntoElement {
        match item {
            Some(id) => {
                let asset = self.pool.read(cx).get(id).unwrap();
                div().child(asset.label().to_string())
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
        let asset = self.pool.read(cx).get(item).unwrap();
        div().child(asset.label().to_string())
    }
}
