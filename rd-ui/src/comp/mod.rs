pub mod stateful;

mod binding;
mod button;
mod icon;
mod labelled;
mod section;
mod titlebar;
mod typo;

pub use binding::*;
pub use button::*;
pub use icon::*;
pub use labelled::*;
pub use section::*;
pub use titlebar::*;
pub use typo::*;

pub const INPUT_SIZE: gpui::Pixels = gpui::px(26.0);

pub trait Identifiable {
    fn id<'a>(&'a self, cx: &'a gpui::App) -> &'a gpui::ElementId;
}

pub trait Labelled {
    fn label(&self) -> Option<&gpui::SharedString>;

    fn set_label(&mut self, label: impl Into<Option<gpui::SharedString>>);

    fn with_label(mut self, label: impl Into<gpui::SharedString>) -> Self
    where
        Self: Sized,
    {
        self.set_label(label.into());
        self
    }

    fn icon(&self) -> Option<IconVariant>;

    fn set_icon(&mut self, icon: impl Into<Option<IconVariant>>);

    fn with_icon(mut self, icon: impl Into<IconVariant>) -> Self
    where
        Self: Sized,
    {
        self.set_icon(icon.into());
        self
    }
}

pub trait Disableable {
    fn disabled(&self, _cx: &gpui::App) -> bool;

    fn set_disabled(&mut self, disabled: bool, cx: &mut gpui::App);

    fn with_disabled(mut self, disabled: bool, cx: &mut gpui::App) -> Self
    where
        Self: Sized,
    {
        self.set_disabled(disabled, cx);
        self
    }
}

pub trait FocusableComponent: gpui::Focusable {
    fn set_focus_handle(&mut self, focus_handle: gpui::FocusHandle, cx: &mut gpui::App)
    where
        Self: Sized;

    fn with_focus_handle(mut self, focus_handle: gpui::FocusHandle, cx: &mut gpui::App) -> Self
    where
        Self: Sized,
    {
        self.set_focus_handle(focus_handle, cx);
        self
    }
}
