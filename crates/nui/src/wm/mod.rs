use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use gpui::prelude::*;
use gpui::{AnyWindowHandle, App, Entity, Global, PromptLevel, SharedString, Window};

mod overlay;
mod window;

pub use overlay::*;
pub use window::*;

use crate::AppExt;
use crate::input::{FieldEvent, TextField};

pub(crate) fn init(cx: &mut App) {
    overlay::init(cx);
}

pub struct WindowManager {
    singleton_windows: HashMap<TypeId, AnyWindowHandle>,
    edited_windows: HashSet<AnyWindowHandle>,
    unclosable_windows: HashSet<AnyWindowHandle>,

    overlays: HashMap<AnyWindowHandle, Vec<Overlay>>,

    quit_when_all_windows_closed: bool,
}

impl WindowManager {
    pub(crate) fn new(cx: &mut App) -> Self {
        cx.on_window_closed(move |cx| {
            if cx.windows().is_empty() && cx.wm().quit_when_all_windows_closed {
                cx.quit();
            }
        })
        .detach();

        Self {
            singleton_windows: HashMap::new(),
            edited_windows: HashSet::new(),
            unclosable_windows: HashSet::new(),

            overlays: HashMap::new(),

            quit_when_all_windows_closed: false,
        }
    }

    pub fn quit_when_all_windows_closed(&mut self, should_quit: bool) {
        self.quit_when_all_windows_closed = should_quit;
    }

    pub fn open_singleton_window<D: WindowDelegate>(&mut self, cx: &mut App) {
        let type_id = TypeId::of::<D>();

        if self.singleton_windows.contains_key(&type_id) {
            // Window is already opened.
            return;
        }

        let handle = WindowWrapper::open(cx, |window, cx| {
            let handle = window.window_handle();

            window.on_window_should_close(cx, move |_, cx| {
                cx.update_wm(|wm, cx| {
                    wm.request_close_singleton_window::<D>(cx);
                    wm.can_close_window(&handle)
                })
            });

            D::create(window, cx)
        });

        self.singleton_windows.insert(type_id, handle.into());
    }

    pub fn request_close_singleton_window<D: WindowDelegate>(&mut self, cx: &mut App) {
        let type_id = TypeId::of::<D>();

        let Some(&handle) = self.singleton_windows.get(&type_id) else { return };

        let close_window = move |handle: AnyWindowHandle, cx: &mut App| {
            cx.defer(move |cx| {
                handle
                    .update(cx, |_, window, _| window.remove_window())
                    .expect("should update window");
                cx.update_wm(|wm, _| wm.singleton_windows.remove(&type_id));
                cx.update_wm(|wm, _| wm.unclosable_windows.remove(&handle));
            });
        };

        let is_edited = self.is_edited(&handle);
        if is_edited {
            cx.defer(move |cx| {
                let answer = handle
                    .update(cx, |_, window, cx| {
                        window.prompt(
                            PromptLevel::Warning,
                            "Window has Unsaved Changes",
                            Some("What do you want to do with the changes?"),
                            &["Save", "Discard", "Keep Editing"],
                            cx,
                        )
                    })
                    .expect("should update window");

                cx.spawn(async move |cx| {
                    let Ok(ix) = answer.await else { return };
                    handle
                        .update(cx, move |view, window, cx| {
                            let wrapper: Entity<WindowWrapper<D>> = view.downcast().unwrap();

                            match ix {
                                0 => {
                                    wrapper.update(cx, |wrapper, cx| {
                                        wrapper.handle_window_save(window, cx);
                                    });
                                    close_window(handle, cx);
                                }
                                1 => {
                                    wrapper.update(cx, |wrapper, cx| {
                                        wrapper.handle_window_discard(window, cx);
                                    });
                                    close_window(handle, cx);
                                }
                                2 => {}
                                _ => {}
                            }
                        })
                        .expect("should update window");
                })
                .detach();
            });
        } else {
            close_window(handle, cx);
        }
    }

    pub fn set_edited(&mut self, window: &mut Window, edited: bool) {
        let handle = window.window_handle();
        window.set_window_edited(edited);

        if edited {
            self.edited_windows.insert(handle);
            self.unclosable_windows.insert(handle);
        } else {
            self.edited_windows.remove(&handle);
            self.unclosable_windows.remove(&handle);
        }
    }

    pub fn is_edited(&self, handle: &AnyWindowHandle) -> bool {
        self.edited_windows.contains(handle)
    }

    fn can_close_window(&self, handle: &AnyWindowHandle) -> bool {
        !self.unclosable_windows.contains(handle)
    }

    pub fn close_overlay(&mut self, id: &str, handle: &AnyWindowHandle) {
        let Some(overlays) = self.overlays.get_mut(&handle) else { return };
        overlays.retain(|o| o.id() != id);
    }

    pub fn open_overlay(&mut self, overlay: Overlay, handle: &AnyWindowHandle) {
        match self.overlays.get_mut(handle) {
            Some(overlays) => {
                overlays.push(overlay);
            }
            None => {
                self.overlays.insert(*handle, vec![overlay]);
            }
        }
    }

    pub(crate) fn window_overlays(&self, handle: &AnyWindowHandle) -> Vec<Overlay> {
        self.overlays.get(&handle).map(|overlays| overlays.to_vec()).unwrap_or_default()
    }

    pub fn open_text_modal<F: Fn(SharedString, &mut Window, &mut App) + 'static>(
        &mut self,
        id: impl Into<String>,
        title: impl Into<SharedString>,
        initial_value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut App,
        on_submit: F,
    ) {
        let id = id.into();

        let modal = cx.new(|cx| TextModal::new(initial_value.into(), window, cx));
        let field = modal.read(cx).field.clone();

        window
            .subscribe(&field, cx, {
                let id = id.clone();
                move |field: Entity<TextField>, event, window, cx| match event {
                    FieldEvent::Submit => {
                        let value = field.read(cx).value(cx).clone();
                        on_submit(value, window, cx);
                        cx.update_wm(|wm, _| wm.close_overlay(&id, &window.window_handle()));
                    }
                    _ => {}
                }
            })
            .detach();

        self.open_overlay(Overlay::new(id, title, modal).as_modal(), &window.window_handle());
    }
}

impl Global for WindowManager {}
