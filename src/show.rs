use std::collections::HashMap;

use gpui::{
    div, white, AppContext, Context, FocusHandle, FocusableView, Global, InteractiveElement,
    IntoElement, Model, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::{layout::Layout, presets::Presets, window::Window};

use super::screen::{Screen, ScreenView};

pub mod cmd {
    use gpui::actions;

    actions!(show_cmd, [Store]);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Show {
    pub presets: Presets,
    pub programmer_state: SharedString,
    screens: HashMap<ScreenId, Screen>,
}

impl Show {
    pub fn new() -> Self {
        Self {
            presets: Presets::new(),
            programmer_state: "normal".into(),
            screens: HashMap::new(),
        }
    }

    pub fn add_screen(&mut self, screen: Screen) -> ScreenId {
        let screen_id = ScreenId(self.screens.len());
        let id = screen_id;
        self.screens.insert(id.clone(), screen);
        id
    }

    pub fn get_screen(&self, id: &ScreenId) -> Option<&Screen> {
        self.screens.get(id)
    }

    pub fn get_screen_mut(&mut self, id: &ScreenId) -> Option<&mut Screen> {
        self.screens.get_mut(id)
    }

    pub fn screens(&self) -> &HashMap<ScreenId, Screen> {
        &self.screens
    }
}

impl Global for Show {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ScreenId(usize);

pub struct ShowView {
    screens: Vec<Model<Screen>>,

    focus_handle: FocusHandle,
}

impl FocusableView for ShowView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ShowView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let screens = Self::get_screens(cx);

            let focus_handle = cx.focus_handle();

            let this = Self {
                screens,
                focus_handle,
            };

            cx.observe_global::<Show>(move |this: &mut Self, cx| {
                this.screens = Self::get_screens(cx);
                cx.notify();
            })
            .detach();

            this
        })
    }

    fn get_screens(cx: &mut AppContext) -> Vec<Model<Screen>> {
        cx.global::<Show>()
            .screens()
            .clone()
            .into_iter()
            .map(|(_id, screen)| cx.new_model(|_cx| screen))
            .collect()
    }

    fn cmd_store(&mut self, _action: &cmd::Store, cx: &mut ViewContext<Self>) {
        println!("Store");
        cx.update_global::<Show, _>(|show, cx| {
            show.add_screen(Screen {
                layout: Layout {
                    windows: vec![Window {}],
                },
            });
            cx.notify();
        })
    }
}

impl Render for ShowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .track_focus(&self.focus_handle)
            .key_context("Show")
            .on_action(cx.listener(Self::cmd_store))
            .font("Zed Sans")
            .text_color(white())
            .children({
                self.screens
                    .iter()
                    .map(|screen| ScreenView::build(screen.clone(), cx))
            })
    }
}
