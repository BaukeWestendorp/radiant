pub fn init(cx: &mut gpui::App) {
    crate::theme::init(cx);
    crate::popup::init(cx);
    crate::settings::init(cx);

    simple::action::init(cx);
}

pub mod simple {
    use gpui::{AnyView, prelude::*};
    use gpui::{
        App, Entity, FocusHandle, FontWeight, Menu, MenuItem, Pixels, QuitMode, SharedString, Size,
        TitlebarOptions, Window, WindowBounds, WindowOptions, div, px, size,
    };

    use crate::{ActiveTheme, Root, TitleBar, h_flex};

    pub(crate) mod action {
        gpui::actions!([Quit]);

        pub(crate) fn init(cx: &mut gpui::App) {
            cx.on_action::<Quit>(|_, cx| cx.quit());
        }
    }

    pub fn build_simple_app() -> SimpleAppBuilder {
        SimpleAppBuilder::new()
    }

    pub struct SimpleAppBuilder {
        window_title: SharedString,
        window_size: Size<Pixels>,
        title_bar_content: Option<Box<dyn FnOnce(&mut Window, &mut App) -> AnyView>>,
        activate: bool,
        config: Option<config::Config>,
    }

    impl Default for SimpleAppBuilder {
        fn default() -> Self {
            Self {
                window_title: "Preview App".into(),
                window_size: size(px(1080.0), px(720.0)),
                title_bar_content: None,
                activate: true,
                config: None,
            }
        }
    }

    impl SimpleAppBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn window_title(mut self, window_title: impl Into<SharedString>) -> Self {
            self.window_title = window_title.into();
            self
        }

        pub fn window_size(mut self, window_size: Size<Pixels>) -> Self {
            self.window_size = window_size;
            self
        }

        pub fn title_bar_content(
            mut self,
            build_content: impl FnOnce(&mut Window, &mut App) -> AnyView + 'static,
        ) -> Self {
            self.title_bar_content = Some(Box::new(build_content));
            self
        }

        pub fn activate(mut self, activate: bool) -> Self {
            self.activate = activate;
            self
        }

        pub fn config(mut self, config: config::Config) -> Self {
            self.config = config.into();
            self
        }

        pub fn run<V>(
            self,
            build_content: impl FnOnce(&mut Window, &mut App) -> Entity<V> + 'static,
        ) where
            V: 'static + Render,
        {
            gpui_platform::application()
                .with_assets(crate::Assets::default())
                .with_quit_mode(QuitMode::LastWindowClosed)
                .run(move |cx: &mut App| {
                    crate::init(cx);
                    crate::keymap::default_keymap().apply(cx);

                    if let Some(config) = self.config {
                        crate::feature::config::init(config, cx);
                    }

                    cx.set_menus([Menu::new("").items([MenuItem::action("Quit", action::Quit)])]);

                    if self.activate {
                        cx.activate(true);
                    }

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
                            let title_bar_content =
                                self.title_bar_content.map(|tbc| (tbc)(window, cx));
                            let view =
                                cx.new(|cx| SimpleAppView::new(content, title_bar_content, cx));
                            cx.new(|cx| Root::new(view, window, cx))
                        },
                    )
                    .unwrap();
                });
        }
    }

    struct SimpleAppView<V: Render + 'static> {
        content: Entity<V>,
        title_bar_content: Option<AnyView>,
        focus_handle: FocusHandle,
    }

    impl<V: Render + 'static> SimpleAppView<V> {
        fn new(
            content: Entity<V>,
            title_bar_content: Option<AnyView>,
            cx: &mut Context<Self>,
        ) -> Self {
            Self { content, title_bar_content, focus_handle: cx.focus_handle() }
        }

        fn render_title_bar_content(
            &mut self,
            window: &mut Window,
            cx: &mut Context<Self>,
        ) -> impl IntoElement {
            h_flex()
                .size_full()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(cx.theme().fg_secondary)
                        .child(window.window_title()),
                )
                .children(self.title_bar_content.clone())
        }

        fn render_content(
            &mut self,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> impl IntoElement {
            self.content.clone()
        }
    }

    impl<V: Render + 'static> Render for SimpleAppView<V> {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .track_focus(&self.focus_handle)
                .flex()
                .flex_col()
                .size_full()
                .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
                .child(div().size_full().overflow_hidden().child(self.render_content(window, cx)))
        }
    }
}
