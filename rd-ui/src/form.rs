use gpui::{
    AnyElement, App, FlexDirection, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, prelude::*,
};
use smallvec::SmallVec;

use crate::{ActiveTheme, section};

pub fn form() -> Form {
    Form::new()
}

#[derive(IntoElement)]
pub struct Form {
    elements: SmallVec<[FormItem; 2]>,
}

impl Form {
    pub fn new() -> Self {
        Self { elements: SmallVec::new() }
    }

    pub fn child(mut self, element: FormItem) -> Self {
        self.elements.push(element);
        self
    }
}

impl RenderOnce for Form {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().tab_group().flex().flex_col().w_full().gap_4().children(self.elements)
    }
}

#[derive(IntoElement)]
pub enum FormItem {
    Section { title: Option<SharedString>, flex_direction: FlexDirection, children: Vec<FormItem> },
    Input { label: Option<SharedString>, input: AnyElement },
}

impl FormItem {
    pub fn section(
        title: Option<&str>,
        flex_direction: FlexDirection,
        children: impl IntoIterator<Item = FormItem>,
    ) -> Self {
        FormItem::Section {
            title: title.map(|s| s.into()),
            flex_direction,
            children: children.into_iter().collect(),
        }
    }

    pub fn input(label: Option<&str>, input: impl IntoElement + 'static) -> Self {
        FormItem::Input { label: label.map(|s| s.into()), input: input.into_any_element() }
    }
}

impl RenderOnce for FormItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        match self {
            FormItem::Section { title, flex_direction, children } => {
                let mut content = div().flex().flex_col().w_full().gap_2().children(children);

                match flex_direction {
                    FlexDirection::Row => content = content.flex_row(),
                    FlexDirection::Column => content = content.flex_col(),
                    FlexDirection::RowReverse => content = content.flex_row_reverse(),
                    FlexDirection::ColumnReverse => content = content.flex_col_reverse(),
                }

                match title {
                    Some(title) => section(title).child(content).into_any_element(),
                    None => content.into_any_element(),
                }
            }
            FormItem::Input { label, input } => div()
                .w_full()
                .items_center()
                .gap_2()
                .when_some(label, |e, label| {
                    e.child(div().text_color(cx.theme().fg_secondary).child(label))
                })
                .child(div().child(input.into_any_element()))
                .into_any_element(),
        }
    }
}
