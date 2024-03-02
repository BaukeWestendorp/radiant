use assets::Assets;
use gpui::{
    App, AppContext, AssetSource, Bounds, Point, Size, VisualContext, WindowBounds, WindowOptions,
};
use palette::Srgb;
use show::{LedGroup, Show};
use ui::{
    pool::{Pool, PoolKind},
    pool_grid::PoolGrid,
};

pub mod show;
pub mod ui;

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

        cx.set_global(Show::new());

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
                let mut pool_grid = PoolGrid::new(5, 10, cx);

                let pool = Pool::new(PoolKind::Color, 2, 8, 0, 0, cx);
                let color = cx.global_mut::<Show>().add_color(Srgb::new(0.8, 0.3, 0.3));
                pool.update(cx, |pool, cx| {
                    pool.insert_cell(2, color, cx);
                });
                pool_grid.add_pool(pool);

                let pool = Pool::new(PoolKind::Group, 2, 6, 0, 3, cx);
                let group = cx
                    .global_mut::<Show>()
                    .add_group(LedGroup::new(vec![0, 1, 2, 3]));
                pool.update(cx, |pool, cx| {
                    pool.insert_cell(2, group, cx);
                });
                pool_grid.add_pool(pool);

                cx.new_view(|_| pool_grid)
            },
        );
    })
}
