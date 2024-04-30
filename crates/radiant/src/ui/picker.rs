use gpui::{
    div, prelude::FluentBuilder, px, ElementId, FlexDirection, IntoElement, Model, ParentElement,
    Render, SharedString, Styled, View, ViewContext, VisualContext, WindowContext,
};

use super::{Button, Disableable, Selectable};

pub struct Picker<V>
where
    V: PartialEq + Clone + 'static,
{
    direction: FlexDirection,
    options: Vec<PickerOption<V>>,
    selected: Model<Option<PickerOption<V>>>,
}

impl<V: PartialEq + Clone + 'static> Picker<V> {
    pub fn build(
        direction: FlexDirection,
        options: Vec<PickerOption<V>>,
        selected: Model<Option<PickerOption<V>>>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self {
            direction,
            options,
            selected,
        })
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
                .size_full()
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

        div()
            .w_full()
            .flex()
            .when(self.direction == FlexDirection::Column, |this| {
                this.flex_col()
            })
            .when(self.direction == FlexDirection::ColumnReverse, |this| {
                this.flex_col_reverse()
            })
            .when(self.direction == FlexDirection::Row, |this| this.flex_row())
            .when(self.direction == FlexDirection::RowReverse, |this| {
                this.flex_row_reverse()
            })
            .gap_2()
            .children(buttons)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PickerOption<V: PartialEq + Clone> {
    pub id: ElementId,
    pub label: SharedString,
    pub value: Option<V>,
}
