use app::RadiantApp;
use gpui::App;

mod app;
mod assets;
mod io;
mod view;

fn main() {
    env_logger::init();

    App::new().with_assets(assets::Assets).run(|cx| {
        RadiantApp::new().run(cx);
    })
}
