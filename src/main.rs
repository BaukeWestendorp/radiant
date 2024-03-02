use assets::Assets;
use gpui::{
    actions, App, AppContext, AssetSource, Bounds, KeyBinding, Point, Size, VisualContext,
    WindowBounds, WindowOptions,
};
use show::Show;
use ui::{
    layout::{Layout, LayoutId},
    window::{ColorPresetWindow, Window, WindowKind},
};

pub mod dmx;
pub mod show;
pub mod ui;

actions!(show, [Save]);

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

        let show = match load_show() {
            Ok(show) => show,
            Err(_) => {
                let mut show = Show::new();
                let main_layout_id = show.add_layout(Layout::new());
                let main_layout = show.layout_mut(main_layout_id);
                main_layout.add_window(Window::new(WindowKind::ColorPreset(
                    ColorPresetWindow::new(),
                )));
                show
            }
        };

        cx.set_global(show);

        cx.bind_keys([KeyBinding::new("cmd-s", Save, None)]);

        cx.on_action::<Save>(|_action, cx| {
            let show = cx.global::<Show>();
            let serialized_show = serde_json::to_string(&show).unwrap();
            std::fs::write("show.json", serialized_show).unwrap();
        });

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Point {
                        x: 500.0.into(),
                        y: 350.0.into(),
                    },
                    size: Size {
                        width: 1280.0.into(),
                        height: 720.0.into(),
                    },
                }),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|cx| {
                    // FIXME: We should get this dynamically from the show.
                    let main_layout_id = LayoutId(0);
                    let main_layout = cx.global::<Show>().layout(main_layout_id);
                    main_layout.clone()
                })
            },
        );
    })
}

fn load_show() -> Result<Show, ()> {
    let serialized_show = std::fs::read_to_string("show.json").map_err(|_| ())?;
    let show = serde_json::from_str(&serialized_show).map_err(|_| ())?;
    Ok(show)
}
