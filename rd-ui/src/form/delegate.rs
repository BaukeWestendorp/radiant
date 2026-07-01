use std::hash::Hash;

use crate::FormState;
use gpui::{App, Context, FlexDirection, SharedString, Window, prelude::*};

pub enum FormNode<Id> {
    Section {
        title: Option<SharedString>,
        flex_direction: FlexDirection,
        children: Vec<FormNode<Id>>,
    },
    Field {
        id: Id,
        label: Option<SharedString>,
    },
    Custom {
        id: Id,
    },
}

impl<Id> FormNode<Id> {
    pub fn section(
        title: impl Into<SharedString>,
        flex_direction: FlexDirection,
        children: impl IntoIterator<Item = FormNode<Id>>,
    ) -> Self {
        Self::Section {
            title: Some(title.into()),
            flex_direction,
            children: children.into_iter().collect(),
        }
    }

    pub fn section_headless(
        flex_direction: FlexDirection,
        children: impl IntoIterator<Item = FormNode<Id>>,
    ) -> Self {
        Self::Section { title: None, flex_direction, children: children.into_iter().collect() }
    }

    pub fn field(id: Id, label: impl Into<SharedString>) -> Self {
        Self::Field { id, label: Some(label.into()) }
    }

    pub fn custom(id: Id) -> Self {
        Self::Custom { id }
    }
}

pub trait FormDelegate: Sized + 'static {
    type Id: Clone + Eq + Hash;
    type Data;

    fn layout(&self, cx: &App) -> Vec<FormNode<Self::Id>>;

    fn render_input(
        &self,
        id: &Self::Id,
        window: &mut Window,
        cx: &mut Context<FormState<Self>>,
    ) -> impl IntoElement;

    fn extract_data(&self, cx: &App) -> Option<Self::Data>;
}
