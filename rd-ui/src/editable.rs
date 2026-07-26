use std::rc::Rc;

use gpui::{App, InteractiveElement, MouseButton, Window};

pub trait Editable: InteractiveElement + Sized {
    fn on_edit(self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        let listener = Rc::new(listener);

        self.on_mouse_down(MouseButton::Right, {
            let listener = listener.clone();
            move |_, window, cx| {
                (listener)(window, cx);
            }
        })
    }
}

impl<E: InteractiveElement> Editable for E {}
