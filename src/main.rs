use assets::Assets;
use gpui::{
    actions, point, size, App, AppContext, AssetSource, Bounds, Context, KeyBinding, VisualContext,
    WindowBounds, WindowOptions,
};
use show::Show;
use workspace::Workspace;

pub mod color;
pub mod dmx;
pub mod show;
pub mod ui;
pub mod workspace;

actions!(app, [Quit]);

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.text_system()
            .add_fonts(vec![
                Assets.load("fonts/zed-sans/zed-sans-extended.ttf").unwrap(),
                Assets
                    .load("fonts/zed-sans/zed-sans-extendedbold.ttf")
                    .unwrap(),
                Assets
                    .load("fonts/zed-sans/zed-sans-extendeditalic.ttf")
                    .unwrap(),
            ])
            .unwrap();

        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-o", workspace::actions::OpenShow, Some("Workspace")),
            KeyBinding::new("s", workspace::actions::cmd::Store, Some("Workspace")),
            KeyBinding::new("escape", workspace::actions::cmd::Clear, Some("Workspace")),
        ]);

        cx.on_action(|_action: &Quit, cx: &mut AppContext| cx.quit());

        let show = cx.new_model(|_cx| Show::default());

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: point(0.0.into(), 0.0.into()),
                    size: size(600.0.into(), 400.0.into()),
                }),
                ..Default::default()
            },
            |cx| cx.new_view(|cx| Workspace::new(show, cx)),
        );
    })
}
