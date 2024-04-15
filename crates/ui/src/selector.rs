use gpui::{
    div, prelude::FluentBuilder, FlexDirection, IntoElement, Model, ParentElement, RenderOnce,
    SharedString, Styled, WindowContext,
};

use crate::{
    button::{Button, ButtonStyle},
    disableable::Disableable,
    selectable::Selectable,
};

#[derive(IntoElement)]
pub struct Selector<V: PartialEq + Clone + 'static> {
    direction: FlexDirection,
    labels: Vec<SharedString>,
    values: Vec<V>,
    selected: Model<Option<V>>,
}

impl<V: PartialEq + Clone + 'static> Selector<V> {
    pub fn new(
        direction: FlexDirection,
        labels: Vec<SharedString>,
        values: Vec<V>,
        selected: Model<Option<V>>,
    ) -> Self {
        Self {
            direction,
            labels,
            values,
            selected,
        }
    }
}

impl<V> RenderOnce for Selector<V>
where
    V: PartialEq + Clone + 'static,
{
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let buttons = self
            .labels
            .into_iter()
            .enumerate()
            .map({
                let selected = self.selected.clone();
                move |(ix, label)| {
                    let value = self.values.get(ix);
                    Button::new(ButtonStyle::Secondary, label.clone(), cx)
                        .size_full()
                        .px_2()
                        .py_1()
                        .disabled(value.is_none())
                        .selected(
                            selected
                                .read(cx)
                                .as_ref()
                                .is_some_and(|selected| Some(selected) == value),
                        )
                        .on_click({
                            let value = value.cloned();
                            let selected = selected.clone();
                            move |_event, cx| {
                                selected.update(cx, |selected, cx| {
                                    *selected = value.clone();
                                    cx.notify();
                                });
                            }
                        })
                        .child(label.to_string())
                }
            })
            .collect::<Vec<_>>();

        div()
            .flex()
            .size_full()
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
