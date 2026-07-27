use std::rc::Rc;

use gpui::{App, FocusHandle, Global, UpdateGlobal, Window};

pub(crate) fn init(cx: &mut App) {
    cx.set_global(EditableGlobal::default());
}

pub trait EditableAppExt {
    fn set_edit_handler<F: Fn(&mut Window, &mut App) + 'static>(
        &mut self,
        focus_handle: &FocusHandle,
        window: &mut Window,
        edit_handler: F,
    );
}

impl EditableAppExt for App {
    fn set_edit_handler<F: Fn(&mut Window, &mut App) + 'static>(
        &mut self,
        focus_handle: &FocusHandle,
        window: &mut Window,
        edit_handler: F,
    ) {
        let edit_handler = Rc::new(edit_handler);
        window
            .on_focus_in(focus_handle, self, {
                move |_, cx| {
                    let edit_handler = Rc::clone(&edit_handler);
                    EditableGlobal::update_global(cx, move |global, _| {
                        global.current_handler = Some(edit_handler);
                    })
                }
            })
            .detach();

        window
            .on_focus_out(focus_handle, self, {
                move |_, _, cx| {
                    EditableGlobal::update_global(cx, move |global, _| {
                        global.current_handler = None;
                    })
                }
            })
            .detach();
    }
}

#[derive(Default)]
pub(crate) struct EditableGlobal {
    pub current_handler: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl Global for EditableGlobal {}
