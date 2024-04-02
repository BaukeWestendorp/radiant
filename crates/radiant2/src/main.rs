use app::run_app;
use gpui::App;

mod app;
mod assets;
mod geometry;
mod layout;
mod output;
mod showfile;
mod window;
mod workspace;

fn main() {
    env_logger::init();
    let app = App::new();
    let showfile_path = Some("example_show");
    run_app(app, showfile_path)
}
