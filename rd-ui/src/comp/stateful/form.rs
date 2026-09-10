use std::sync::Arc;

use gpui::{AnyView, App, Entity, EventEmitter, SharedString, Window, prelude::*};

use crate::{
    comp::{
        Icon, IconSize, IconVariant, Labelled,
        stateful::{InputEvent, InputValue},
    },
    h_flex, v_flex,
};

pub struct Form<Data> {
    data: Entity<Data>,
    inputs: Vec<Entity<FormInput<Data>>>,
}

impl<Data: 'static> Form<Data> {
    pub fn new(data: Entity<Data>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { data, inputs: Vec::new() }
    }

    pub fn data(&self) -> Entity<Data> {
        self.data.clone()
    }

    pub fn add_input(&mut self, input: FormInput<Data>, cx: &mut App) {
        (input.push_to_view)(self.data.clone(), cx);
        self.inputs.push(cx.new(|_| input));
    }

    pub fn with_input(mut self, input: FormInput<Data>, cx: &mut App) -> Self {
        self.add_input(input, cx);
        self
    }
}

impl<Data: 'static> Render for Form<Data> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().gap_2().children(self.inputs.iter().map(|input| input.clone()))
    }
}

pub struct FormInput<Data> {
    pub view: AnyView,
    pub label: Option<SharedString>,
    pub icon: Option<IconVariant>,
    pub push_to_view: Box<dyn Fn(Entity<Data>, &mut App)>,
    _data: std::marker::PhantomData<Data>,
}

impl<Data: 'static> FormInput<Data> {
    pub fn new<T, V>(
        view: Entity<T>,
        key_path: KeyPath<Data, V>,
        cx: &mut Context<Form<Data>>,
    ) -> Self
    where
        T: Render + FormWidget<V> + 'static,
        V: Clone + 'static,
    {
        cx.subscribe(&view, {
            let key_path = key_path.clone();
            move |form, _, event: &InputEvent<V>, cx| {
                form.data.update(cx, |data, cx| match event {
                    InputEvent::Submit(value) => {
                        (key_path.setter)(data, value.clone());
                        cx.notify();
                    }
                    InputEvent::Change(value) => {
                        let InputValue::Valid(value) = value else { return };
                        (key_path.setter)(data, value.clone());
                        cx.notify();
                    }
                })
            }
        })
        .detach();

        // FIXME: Oefff
        let push_to_view = Box::new({
            let view = view.clone();
            let key_path = key_path.clone();
            move |data: Entity<Data>, cx: &mut App| {
                let InputValue::Valid(initial_value) = (key_path.getter)(data.read(cx)) else {
                    return;
                };
                view.update(cx, |this, cx| {
                    this.set_value(initial_value, cx);
                    cx.notify();
                });
            }
        }) as Box<dyn Fn(Entity<Data>, &mut App)>;

        Self {
            view: view.into(),
            label: None,
            icon: None,
            push_to_view,
            _data: std::marker::PhantomData,
        }
    }
}

impl<Data: 'static> Render for FormInput<Data> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .justify_between()
            .w_full()
            .when_some(self.icon, |e, icon| e.child(Icon::new(icon, IconSize::Small)))
            .when_some(self.label.as_ref(), |e, label| e.child(label.clone().into_element()))
            .child(self.view.clone())
    }
}

impl<Data: 'static> Labelled for FormInput<Data> {
    fn label(&self) -> Option<&SharedString> {
        self.label.as_ref()
    }

    fn set_label(&mut self, label: impl Into<Option<SharedString>>) {
        self.label = label.into();
    }

    fn icon(&self) -> Option<IconVariant> {
        self.icon
    }

    fn set_icon(&mut self, icon: impl Into<Option<IconVariant>>) {
        self.icon = icon.into();
    }
}

pub struct KeyPath<Data, V> {
    pub getter: Arc<dyn Fn(&Data) -> InputValue<V>>,
    pub setter: Arc<dyn Fn(&mut Data, V)>,
}

impl<Data, V> Clone for KeyPath<Data, V> {
    fn clone(&self) -> Self {
        Self { getter: Arc::clone(&self.getter), setter: Arc::clone(&self.setter) }
    }
}

impl<Data, V> KeyPath<Data, V> {
    pub fn new(
        getter: impl Fn(&Data) -> InputValue<V> + 'static,
        setter: impl Fn(&mut Data, V) + 'static,
    ) -> Self {
        Self { getter: Arc::new(getter), setter: Arc::new(setter) }
    }
}

pub trait FormWidget<V: 'static>: EventEmitter<InputEvent<V>> + Sized {
    fn value(&self, cx: &App) -> InputValue<V>;
    fn set_value(&mut self, value: V, cx: &mut Context<Self>);
}
