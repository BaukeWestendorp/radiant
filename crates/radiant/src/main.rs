use crate::view::project::ProjectView;
use gpui::*;
use show::Show;
use ui::theme::Theme;

mod assets;
mod view;

actions!(app, [Quit]);

fn main() {
    env_logger::init();

    App::new().with_assets(assets::Assets).run(|cx| {
        cx.set_global(Theme::default());
        ui::init(cx);
        flow_gpui::init(cx);

        let show = Show::default();

        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.set_menus(vec![Menu {
            name: "Radiant".to_string().into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1200.0), px(800.0)),
                cx,
            ))),
            ..Default::default()
        };

        register_actions(cx);

        cx.open_window(options, |cx| ProjectView::build(show, cx))
            .unwrap();

        cx.activate(true);
    })
}

fn register_actions(cx: &mut AppContext) {
    cx.on_action::<Quit>(|_action, cx| {
        cx.quit();
    });
}
