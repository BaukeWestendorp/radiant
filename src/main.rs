use assets::Assets;
use dmx::color::DmxColor;
use gpui::{
    actions, point, size, App, AppContext, AssetSource, Bounds, Context, KeyBinding, VisualContext,
    WindowBounds, WindowOptions,
};
use show::presets::ColorPreset;
use show::{ColorPickerWindow, Show};
use workspace::layout::{LayoutBounds, LayoutPoint, LayoutSize};
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
                    size: size(800.0.into(), 600.0.into()),
                }),
                ..Default::default()
            },
            |cx| {
                let workspace = cx.new_view(|cx| Workspace::new(show.clone(), cx));

                show.update(cx, |show, cx| {
                    let mut new_show = Show::default();
                    new_show.name = "Super mega show".into();
                    new_show.layout.add_window(show::Window {
                        bounds: LayoutBounds::new(LayoutPoint::new(0, 0), LayoutSize::new(8, 4)),
                        kind: show::WindowKind::ColorPicker(ColorPickerWindow {}),
                    });
                    new_show
                        .presets
                        .add_color_preset(ColorPreset::new("Green", DmxColor::new(0, 255, 0)));
                    *show = new_show;
                    cx.notify();
                });

                workspace
            },
        );
    })
}
