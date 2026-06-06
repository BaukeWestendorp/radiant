use gpui::{AnyElement, App, Bounds, Entity, FontWeight, SharedString, Window, div, prelude::*};
use rd_engine::gdtf::attr::{Attribute, AttributeName};
use rd_ui::{ActiveTheme as _, Button, TileDelegate, h_flex, h1, h3, v_flex};

use crate::engine::EngineManager;

pub struct AttributeEditorTile {
    selected_attributes: Entity<Vec<Attribute>>,
}

impl AttributeEditorTile {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        Self { selected_attributes: cx.new(|_| Vec::new()) }
    }

    fn select(
        selected_attributes: &Entity<Vec<Attribute>>,
        attribute_name: AttributeName,
        cx: &mut App,
    ) {
        let snapshot = EngineManager::snapshot(cx);
        let mut attributes = Vec::new();
        for gdtf in snapshot.selection().unique_gdtds(snapshot.patch()) {
            let Some(attribute) = gdtf.attribute(&attribute_name) else { continue };
            attributes.push(attribute.clone());
        }

        selected_attributes.write(cx, attributes);
    }

    fn render_attribute_tree(&self, _window: &mut Window, cx: &App) -> impl IntoElement {
        let tree = EngineManager::snapshot(cx).selection().attribute_tree();
        div().flex().size_full().children(tree.feature_names().map({
            let selected_attributes = self.selected_attributes.clone();
            move |feature_name| {
                let attributes = tree.attributes(feature_name);
                v_flex()
                    .size_full()
                    .child(
                        Button::new(feature_name.to_string())
                            .disabled(true)
                            .child(feature_name.to_string()),
                    )
                    .children(attributes.map({
                        let selected_attributes = selected_attributes.clone();
                        move |attribute_name| {
                            let selected_attributes = selected_attributes.clone();
                            Button::new(attribute_name.to_string())
                                .disabled(false)
                                .on_click({
                                    let selected_attributes = selected_attributes.clone();
                                    let attribute_name = attribute_name.clone();
                                    move |_, _, cx| {
                                        Self::select(
                                            &selected_attributes,
                                            attribute_name.clone(),
                                            cx,
                                        );
                                    }
                                })
                                .child(attribute_name.to_string())
                        }
                    }))
            }
        }))
    }

    fn render_value_editor(&self, _window: &mut Window, cx: &App) -> impl IntoElement {
        let attributes = self.selected_attributes.read(cx);

        if attributes.is_empty() {
            return div();
        }

        let attribute_name = attributes.first().unwrap().name();

        let header = h_flex()
            .h_10()
            .px_2()
            .border_b_1()
            .border_color(cx.theme().border_primary)
            .child(div().font_weight(FontWeight::BOLD).child(attribute_name.to_string()));

        let values = div();

        v_flex().size_full().child(header).child(values)
    }
}

impl TileDelegate for AttributeEditorTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Attribute Editor".into()
    }

    fn render_content(&self, _bounds: Bounds<u32>, window: &mut Window, cx: &App) -> AnyElement {
        h_flex()
            .size_full()
            .child(self.render_attribute_tree(window, cx))
            .child(
                div()
                    .w_1_3()
                    .h_full()
                    .border_l_1()
                    .border_color(cx.theme().border_primary)
                    .child(self.render_value_editor(window, cx)),
            )
            .into_any_element()
    }
}
