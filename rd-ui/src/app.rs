use gpui::{
    AnyView, App, Entity, FocusHandle, Focusable, Pixels, QuitMode, SharedString, Size,
    TitlebarOptions, Window, WindowBounds, WindowOptions, div, prelude::*, px, size,
};

use crate::{ActiveTheme, Keymap, Root, StyledExt, comp::TitleBar, h_flex};

pub(crate) mod action {
    gpui::actions!(app, [Quit]);
}

pub struct AppBuilder {
    window_title: SharedString,
    window_size: Size<Pixels>,
    quit_mode: QuitMode,
    activated: bool,
    keymap: Keymap,
    settings_window_content: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            window_title: "RD-UI Application".into(),
            window_size: size(px(1080.0), px(720.0)),
            quit_mode: QuitMode::LastWindowClosed,
            activated: true,
            keymap: Keymap::default(),
            settings_window_content: None,
        }
    }
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_window_title(mut self, window_title: impl Into<SharedString>) -> Self {
        self.window_title = window_title.into();
        self
    }

    pub fn with_window_size(mut self, window_size: Size<Pixels>) -> Self {
        self.window_size = window_size;
        self
    }

    pub fn with_quit_mode(mut self, quit_mode: QuitMode) -> Self {
        self.quit_mode = quit_mode;
        self
    }

    pub fn with_activated(mut self, activated: bool) -> Self {
        self.activated = activated;
        self
    }

    pub fn with_keymap(mut self, keymap: Keymap) -> Self {
        self.keymap = keymap;
        self
    }

    pub fn with_settings_window_content(
        mut self,
        settings_window_content: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.settings_window_content = Some(Box::new(settings_window_content));
        self
    }

    pub fn run<V>(self, build_content: impl FnOnce(&mut Window, &mut App) -> Entity<V> + 'static)
    where
        V: 'static + Render,
    {
        gpui_platform::application()
            .with_assets(crate::Assets::default())
            .with_quit_mode(self.quit_mode)
            .run(move |cx: &mut App| {
                crate::init(cx);

                self.keymap.apply(cx);

                if self.activated {
                    cx.activate(true);
                }

                cx.on_action::<action::Quit>(|_, cx| cx.quit());

                cx.open_window(
                    WindowOptions {
                        titlebar: Some(TitlebarOptions {
                            title: Some(self.window_title),
                            appears_transparent: true,
                            ..Default::default()
                        }),
                        window_bounds: Some(WindowBounds::centered(self.window_size, cx)),
                        ..Default::default()
                    },
                    |window, cx| {
                        let content = (build_content)(window, cx);

                        let view = cx.new(|cx| AppView::new(content, cx));

                        cx.new(|cx| {
                            let mut root = Root::new(view, window, cx);
                            root.set_settings_window_content(self.settings_window_content);
                            root.focus_handle(cx).focus(window, cx);
                            root
                        })
                    },
                )
                .unwrap();
            });
    }
}

struct AppView<V: Render + 'static> {
    content: Entity<V>,
    focus_handle: FocusHandle,
}

impl<V: Render + 'static> AppView<V> {
    fn new(content: Entity<V>, cx: &mut Context<Self>) -> Self {
        Self { content, focus_handle: cx.focus_handle() }
    }

    fn render_title_bar_content(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex().size_full().justify_between().gap_2().child(
            div().font_bold().text_color(cx.theme().fg_secondary).child(window.window_title()),
        )
    }
}

impl<V: Render + 'static> Render for AppView<V> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
            .child(div().size_full().overflow_hidden().child(self.content.clone()))
    }
}
