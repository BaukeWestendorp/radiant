use std::path::PathBuf;

use gpui::{App, Application};
use radiant::engine::Engine;

use crate::main_window::MainWindow;
use crate::state::{self};

pub fn run(showfile_path: Option<PathBuf>) {
    let engine = Engine::new(showfile_path.as_ref()).expect("failed to create engine");

    Application::new().with_assets(ui::Assets).run(move |cx: &mut App| {
        cx.activate(true);

        ui::init(cx).expect("failed to initialize ui crate");
        actions::init(cx);

        state::init(engine, cx);

        MainWindow::open(cx).expect("failed to open main window");
    });
}

mod actions {
    use gpui::{App, KeyBinding};
    use radiant::engine::{Keyword, Parameter};
    use radiant::show::ObjectKind;

    use crate::state::{exec_current_cmd_and_log_err, process_cmd_param};

    gpui::actions!(app, [RunCommand]);
    gpui::actions!(
        cmd,
        [
            Clear,
            Store,
            Group,
            Executor,
            Sequence,
            PresetDimmer,
            PresetPosition,
            PresetGobo,
            PresetColor,
            PresetBeam,
            PresetFocus,
            PresetShapers,
            PresetControl,
            PresetVideo
        ]
    );

    pub fn init(cx: &mut App) {
        bind_keys(cx);
        bind_actions(cx);
    }

    fn bind_keys(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("enter", RunCommand, None),
            KeyBinding::new("escape", Clear, None),
            KeyBinding::new("s", Store, None),
            KeyBinding::new("G", Group, None),
            KeyBinding::new("E", Executor, None),
            KeyBinding::new("S", Sequence, None),
            KeyBinding::new("p d", PresetDimmer, None),
            KeyBinding::new("p p", PresetPosition, None),
            KeyBinding::new("p g", PresetGobo, None),
            KeyBinding::new("p c", PresetColor, None),
            KeyBinding::new("p b", PresetBeam, None),
            KeyBinding::new("p f", PresetFocus, None),
            KeyBinding::new("p s", PresetShapers, None),
            KeyBinding::new("p c", PresetControl, None),
            KeyBinding::new("p v", PresetVideo, None),
        ]);
    }

    fn bind_actions(cx: &mut App) {
        cx.on_action::<RunCommand>(|_, cx| exec_current_cmd_and_log_err(cx));
        cx.on_action::<Clear>(|_, cx| process_cmd_param(Keyword::Clear, cx));
        cx.on_action::<Store>(|_, cx| process_cmd_param(Keyword::Store, cx));
        cx.on_action::<Group>(|_, cx| process_cmd_param(ObjectKind::Group, cx));
        cx.on_action::<Executor>(|_, cx| process_cmd_param(ObjectKind::Executor, cx));
        cx.on_action::<Sequence>(|_, cx| process_cmd_param(ObjectKind::Sequence, cx));
        cx.on_action::<PresetDimmer>(|_, cx| process_cmd_param(ObjectKind::PresetDimmer, cx));
        cx.on_action::<PresetPosition>(|_, cx| process_cmd_param(ObjectKind::PresetPosition, cx));
        cx.on_action::<PresetGobo>(|_, cx| process_cmd_param(ObjectKind::PresetGobo, cx));
        cx.on_action::<PresetColor>(|_, cx| process_cmd_param(ObjectKind::PresetColor, cx));
        cx.on_action::<PresetBeam>(|_, cx| process_cmd_param(ObjectKind::PresetBeam, cx));
        cx.on_action::<PresetFocus>(|_, cx| process_cmd_param(ObjectKind::PresetFocus, cx));
        cx.on_action::<PresetShapers>(|_, cx| process_cmd_param(ObjectKind::PresetShapers, cx));
        cx.on_action::<PresetControl>(|_, cx| process_cmd_param(ObjectKind::PresetControl, cx));
        cx.on_action::<PresetVideo>(|_, cx| process_cmd_param(ObjectKind::PresetVideo, cx));
        cx.observe_keystrokes(|event, _, cx| match event.keystroke.key.as_str() {
            key @ ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9") => {
                let num = key.parse().unwrap();
                process_cmd_param(Parameter::Integer(num), cx);
            }
            _ => {}
        })
        .detach();
    }
}
