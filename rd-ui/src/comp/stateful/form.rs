use std::sync::Arc;

use gpui::{AnyView, App, Entity, EventEmitter, Window, prelude::*};

use crate::{comp::stateful, v_flex};

pub struct Form<Data> {
    data: Entity<Data>,
    inputs: Vec<FormInput<Data>>,
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
        self.inputs.push(input);
    }

    pub fn with_input(mut self, input: FormInput<Data>, cx: &mut App) -> Self {
        self.add_input(input, cx);
        self
    }
}

impl<Data: 'static> Render for Form<Data> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().gap_2().children(self.inputs.iter().map(|input| input.view.clone()))
    }
}

pub struct FormInput<Data> {
    pub view: AnyView,
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
        V: 'static,
    {
        cx.subscribe(&view, {
            let key_path = key_path.clone();
            move |form, view, _: &stateful::event::Submit<V>, cx| {
                form.data.update(cx, |data, cx| {
                    let new_value = view.read(cx).get_value(cx);
                    (key_path.setter)(data, new_value);
                    cx.notify();
                })
            }
        })
        .detach();

        // FIXME: Oefff
        let push_to_view = Box::new({
            let view = view.clone();
            let key_path = key_path.clone();
            move |data: Entity<Data>, cx: &mut App| {
                let initial_value = (key_path.getter)(data.read(cx));
                view.update(cx, |this, cx| {
                    this.set_value(initial_value, cx);
                    cx.notify();
                });
            }
        }) as Box<dyn Fn(Entity<Data>, &mut App)>;

        Self { view: view.into(), push_to_view, _data: std::marker::PhantomData }
    }
}

pub struct KeyPath<Data, V> {
    pub getter: Arc<dyn Fn(&Data) -> V>,
    pub setter: Arc<dyn Fn(&mut Data, V)>,
}

impl<Data, V> Clone for KeyPath<Data, V> {
    fn clone(&self) -> Self {
        Self { getter: Arc::clone(&self.getter), setter: Arc::clone(&self.setter) }
    }
}

impl<Data, V> KeyPath<Data, V> {
    pub fn new(
        getter: impl Fn(&Data) -> V + 'static,
        setter: impl Fn(&mut Data, V) + 'static,
    ) -> Self {
        Self { getter: Arc::new(getter), setter: Arc::new(setter) }
    }
}

pub trait FormWidget<V: 'static>: EventEmitter<stateful::event::Submit<V>> + Sized {
    fn get_value(&self, cx: &App) -> V;
    fn set_value(&mut self, value: V, cx: &mut Context<Self>);
}
