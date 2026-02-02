use gpui::{AnyView, Window, div, prelude::*};

pub(crate) mod action {
    use gpui::{App, KeyBinding, actions};

    actions!(root, [Tab, TabPrev]);

    pub const KEY_CONTEXT: &str = "Root";

    pub fn init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("tab", Tab, Some(KEY_CONTEXT)),
            KeyBinding::new("shift-tab", TabPrev, Some(KEY_CONTEXT)),
        ]);
    }
}

pub struct Root {
    view: AnyView,
}

impl Root {
    pub fn new(view: impl Into<AnyView>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { view: view.into() }
    }

    pub fn view(&self) -> &AnyView {
        &self.view
    }

    fn handle_action_tab(&mut self, _: &action::Tab, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }

    fn handle_action_tab_prev(
        &mut self,
        _: &action::TabPrev,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
    }
}

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.set_rem_size(cx.theme().font_size);

        div()
            .id("root")
            .key_context(action::KEY_CONTEXT)
            .on_action(cx.listener(Self::handle_action_tab))
            .on_action(cx.listener(Self::handle_action_tab_prev))
            .relative()
            .size_full()
            .font_family(cx.theme().font_family.clone())
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.view.clone())
    }
}
