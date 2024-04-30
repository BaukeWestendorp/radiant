use gpui::{
    div, px, ElementId, IntoElement, Model, ParentElement, Render, SharedString, Styled, View,
    ViewContext, VisualContext, WindowContext,
};

use super::{Button, Disableable, Selectable};

pub struct Picker<V>
where
    V: PartialEq + Clone + 'static,
{
    options: Vec<PickerOption<V>>,
    selected: Model<Option<PickerOption<V>>>,
}

impl<V: PartialEq + Clone + 'static> Picker<V> {
    pub fn build(
        options: Vec<PickerOption<V>>,
        selected: Model<Option<PickerOption<V>>>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self { options, selected })
    }
}

impl<V: PartialEq + Clone + 'static> Render for Picker<V> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let selected_option = self.selected.read(cx);

        let buttons = self.options.clone().into_iter().map(|option| {
            let disabled = option.value == None;
            let selected = selected_option.as_ref().is_some_and(|o| o.id == option.id);
            let label = option.label.clone();

            Button::new(option.id.clone())
                .disabled(disabled)
                .selected(selected)
                .child(div().px_2().py(px(2.0)).child(label))
                .on_click(cx.listener(move |this, _event, cx| {
                    let option = option.clone();
                    this.selected.update(cx, move |selected, cx| {
                        *selected = Some(option);
                        cx.notify();
                    });
                }))
        });

        div().flex().flex_col().gap_2().children(buttons)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PickerOption<V: PartialEq + Clone> {
    pub id: ElementId,
    pub label: SharedString,
    pub value: Option<V>,
}
