use anyhow::Result;
use gpui::{
    App, Application, Bounds, Context, Entity, TitlebarOptions, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};
use rui::{Root, TitleBar, h_flex};
use zeevonk::{Zeevonk, project::file::ProjectFile};

pub fn run(zv_project_file: ProjectFile) -> Result<()> {
    Application::new().run(|cx: &mut App| {
        rui::init(cx);

        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(1080.0), px(720.0)), cx);
        let options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Radiant".into()),
                appears_transparent: true,
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        cx.open_window(options, |window, cx| {
            let view =
                cx.new(|cx| RadiantApp::new(zv_project_file, cx).expect("should create app"));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("should open main window");
    });

    Ok(())
}

struct RadiantApp {
    _zeevonk: Entity<Zeevonk>,
}

impl RadiantApp {
    pub fn new(zv_project_file: ProjectFile, cx: &mut Context<Self>) -> Result<Self> {
        let zeevonk = Zeevonk::new(zv_project_file)?;
        zeevonk.start();
        Ok(Self { _zeevonk: cx.new(|_| zeevonk) })
    }

    fn render_title_bar_content(
        &mut self,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex().size_full().justify_between().child(window.window_title())
    }
}

impl Render for RadiantApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
            .child("Radiant App")
    }
}
