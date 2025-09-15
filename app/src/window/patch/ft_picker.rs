use gpui::prelude::*;
use gpui::{Entity, EventEmitter, MouseButton, Window, div};
use radiant::builtin::GdtfFixtureTypeId;
use ui::interactive::event::SubmitEvent;
use ui::nav::tabs::{Tab, Tabs};
use ui::theme::{ActiveTheme, InteractiveColor};

use crate::engine::EngineManager;

pub struct FixtureTypePicker {
    tabs: Entity<Tabs>,
}

impl FixtureTypePicker {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let picker = cx.entity();
        let from_showfile = cx.new(|_| FromShowfileTab::new(picker));
        let from_library = cx.new(|_| FromLibraryTab {});

        let tabs = cx.new(|cx| {
            Tabs::new(
                vec![
                    Tab::new("from_showfile", "From Showfile", from_showfile.into()),
                    Tab::new("from_library", "From Library", from_library.into()),
                ],
                window,
                cx,
            )
        });

        Self { tabs }
    }
}

impl Render for FixtureTypePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().bg(cx.theme().background).size_full().child(self.tabs.clone())
    }
}

impl EventEmitter<SubmitEvent<GdtfFixtureTypeId>> for FixtureTypePicker {}

struct FromShowfileTab {
    picker: Entity<FixtureTypePicker>,
}

impl FromShowfileTab {
    pub fn new(picker: Entity<FixtureTypePicker>) -> Self {
        Self { picker }
    }
}

impl Render for FromShowfileTab {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let fts = EngineManager::read_patch(cx, |patch| {
            patch
                .fixture_types()
                .iter()
                .map(|(&id, ft)| {
                    div()
                        .child(ft.long_name.clone())
                        .hover(|e| e.text_color(cx.theme().foreground.hovered()))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.picker.update(cx, |_, cx| {
                                    cx.emit(SubmitEvent { value: id });
                                    cx.notify();
                                });
                            }),
                        )
                })
                .collect::<Vec<_>>()
        });

        div().children(fts)
    }
}

struct FromLibraryTab {}

impl Render for FromLibraryTab {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("TODO")
    }
}
