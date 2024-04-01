use app::run_app;
use gpui::App;

mod app;
mod output;
mod window;
mod workspace;

fn main() {
    env_logger::init();
    let app = App::new();
    run_app(app)
}
