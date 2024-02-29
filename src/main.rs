use assets::Assets;
use gpui::{
    App, AppContext, AssetSource, Bounds, Point, Size, VisualContext, WindowBounds, WindowOptions,
};
use ui::{pool::Pool, pool_grid::PoolGrid};

pub mod ui;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.text_system()
            .add_fonts(vec![Assets
                .load("fonts/zed-sans/zed-sans-extended.ttf")
                .unwrap()])
            .unwrap();

        let window_options = WindowOptions {
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
        };

        cx.open_window(window_options, |cx| {
            let mut pool_grid = PoolGrid::new(cx, 5, 10);

            pool_grid.add_pool(cx.new_view(|cx| Pool::new(cx, 2, 8)));

            cx.new_view(|_| pool_grid)
        });
    })
}
