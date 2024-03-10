use assets::Assets;
use gpui::{
    actions, point, size, App, AppContext, AssetSource, Bounds, Context, KeyBinding, VisualContext,
    WindowBounds, WindowOptions,
};
use show::Show;
use workspace::{cmd, Workspace};

pub mod color;
pub mod dmx;
pub mod show;
pub mod ui;
pub mod workspace;

actions!(app, [Quit, OpenShow]);

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
            KeyBinding::new("cmd-o", OpenShow, None),
            KeyBinding::new("s", cmd::Store, Some("Workspace")),
            KeyBinding::new("escape", cmd::Clear, Some("Workspace")),
        ]);

        cx.on_action(|_action: &Quit, cx: &mut AppContext| cx.quit());

        let show = cx.new_model(|_cx| Show::default());

        cx.on_action({
            let show = show.clone();
            move |_action: &OpenShow, cx: &mut AppContext| {
                show.update(cx, |show, cx| {
                    let mut new_show = Show::default();
                    new_show.name = "Super mega show".into();
                    *show = new_show;
                    cx.notify();
                })
            }
        });

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
