use std::path::PathBuf;

use gpui::{Action, App, Application, KeyBinding};
use radiant::engine::{Engine, Keyword};
use radiant::show::ObjectKind;

use crate::main_window::MainWindow;
use crate::state::{self, exec_current_cmd_and_log_err, process_cmd_param};

pub mod actions {
    gpui::actions!(app, [RunCommand]);
    gpui::actions!(
        cmd,
        [
            Clear,
            Store,
            Update,
            Delete,
            Rename,
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
            PresetVideo,
            Save,
        ]
    );
}

pub fn run(showfile_path: Option<PathBuf>) {
    let engine = Engine::new(showfile_path.as_ref()).expect("failed to create engine");

    Application::new().with_assets(ui::assets::Assets).run(move |cx: &mut App| {
        cx.activate(true);

        ui::init(cx).expect("failed to initialize ui crate");
        state::init(engine, cx);

        cx.bind_keys([
            KeyBinding::new("enter", actions::RunCommand, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("escape", actions::Clear, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("s", actions::Store, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("u", actions::Update, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("d", actions::Delete, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("r", actions::Rename, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("f2", actions::Rename, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("G", actions::Group, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("E", actions::Executor, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("S", actions::Sequence, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p d", actions::PresetDimmer, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p p", actions::PresetPosition, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p g", actions::PresetGobo, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p c", actions::PresetColor, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p b", actions::PresetBeam, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p f", actions::PresetFocus, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p s", actions::PresetShapers, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p c", actions::PresetControl, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("p v", actions::PresetVideo, Some("MainWindow && cmd_allowed")),
            KeyBinding::new("secondary-s", actions::Save, Some("MainWindow && cmd_allowed")),
        ]);

        fn register_global_action<A: Action, F: Fn(&mut App) + 'static>(cx: &mut App, f: F) {
            cx.on_action::<A>(move |_, cx| f(cx));
        }

        register_global_action::<actions::RunCommand, _>(cx, |cx| exec_current_cmd_and_log_err(cx));
        register_global_action::<actions::Clear, _>(cx, |cx| process_cmd_param(Keyword::Clear, cx));
        register_global_action::<actions::Store, _>(cx, |cx| process_cmd_param(Keyword::Store, cx));
        register_global_action::<actions::Update, _>(cx, |cx| {
            process_cmd_param(Keyword::Update, cx)
        });
        register_global_action::<actions::Delete, _>(cx, |cx| {
            process_cmd_param(Keyword::Delete, cx)
        });
        register_global_action::<actions::Rename, _>(cx, |cx| {
            process_cmd_param(Keyword::Rename, cx)
        });
        register_global_action::<actions::Group, _>(cx, |cx| {
            process_cmd_param(ObjectKind::Group, cx)
        });
        register_global_action::<actions::Executor, _>(cx, |cx| {
            process_cmd_param(ObjectKind::Executor, cx)
        });
        register_global_action::<actions::Sequence, _>(cx, |cx| {
            process_cmd_param(ObjectKind::Sequence, cx)
        });
        register_global_action::<actions::PresetDimmer, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetDimmer, cx)
        });
        register_global_action::<actions::PresetPosition, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetPosition, cx)
        });
        register_global_action::<actions::PresetGobo, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetGobo, cx)
        });
        register_global_action::<actions::PresetColor, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetColor, cx)
        });
        register_global_action::<actions::PresetBeam, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetBeam, cx)
        });
        register_global_action::<actions::PresetFocus, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetFocus, cx)
        });
        register_global_action::<actions::PresetShapers, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetShapers, cx)
        });
        register_global_action::<actions::PresetControl, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetControl, cx)
        });
        register_global_action::<actions::PresetVideo, _>(cx, |cx| {
            process_cmd_param(ObjectKind::PresetVideo, cx)
        });
        register_global_action::<actions::Save, _>(cx, |cx| process_cmd_param(Keyword::Save, cx));
        // .observe_keystrokes(|event, _, cx|
        //     match event.keystroke.key.as_str() {
        //         key @ ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" |
        // "9") => {             let num = key.parse().unwrap();
        //             process_cmd_param(Parameter::Integer(num), cx);
        //         }
        //         _ => {}
        //     }
        // )

        MainWindow::open(cx).expect("failed to open main window");
    });
}
