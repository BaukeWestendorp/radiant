use std::path::PathBuf;

use gpui::{Action, App, Application, KeyBinding};
use radiant::engine::{Engine, Keyword, Parameter};
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
            Num0,
            Num1,
            Num2,
            Num3,
            Num4,
            Num5,
            Num6,
            Num7,
            Num8,
            Num9
        ]
    );
}

fn init(cx: &mut App) {
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
        KeyBinding::new("0", actions::Num0, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("1", actions::Num1, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("2", actions::Num2, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("3", actions::Num3, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("4", actions::Num4, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("5", actions::Num5, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("6", actions::Num6, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("7", actions::Num7, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("8", actions::Num8, Some("MainWindow && cmd_allowed")),
        KeyBinding::new("9", actions::Num9, Some("MainWindow && cmd_allowed")),
    ]);

    fn bind_cmd_param_action<A: Action>(cx: &mut App, param: impl Into<Parameter>) {
        cx.on_action::<A>({
            let param = param.into();
            move |_, cx| process_cmd_param(param.clone(), cx)
        });
    }

    cx.on_action::<actions::RunCommand>(|_, cx| exec_current_cmd_and_log_err(cx));
    bind_cmd_param_action::<actions::Clear>(cx, Keyword::Clear);
    bind_cmd_param_action::<actions::Store>(cx, Keyword::Store);
    bind_cmd_param_action::<actions::Update>(cx, Keyword::Update);
    bind_cmd_param_action::<actions::Delete>(cx, Keyword::Delete);
    bind_cmd_param_action::<actions::Rename>(cx, Keyword::Rename);
    bind_cmd_param_action::<actions::Group>(cx, ObjectKind::Group);
    bind_cmd_param_action::<actions::Executor>(cx, ObjectKind::Executor);
    bind_cmd_param_action::<actions::Sequence>(cx, ObjectKind::Sequence);
    bind_cmd_param_action::<actions::PresetDimmer>(cx, ObjectKind::PresetDimmer);
    bind_cmd_param_action::<actions::PresetPosition>(cx, ObjectKind::PresetPosition);
    bind_cmd_param_action::<actions::PresetGobo>(cx, ObjectKind::PresetGobo);
    bind_cmd_param_action::<actions::PresetColor>(cx, ObjectKind::PresetColor);
    bind_cmd_param_action::<actions::PresetBeam>(cx, ObjectKind::PresetBeam);
    bind_cmd_param_action::<actions::PresetFocus>(cx, ObjectKind::PresetFocus);
    bind_cmd_param_action::<actions::PresetShapers>(cx, ObjectKind::PresetShapers);
    bind_cmd_param_action::<actions::PresetControl>(cx, ObjectKind::PresetControl);
    bind_cmd_param_action::<actions::PresetVideo>(cx, ObjectKind::PresetVideo);
    bind_cmd_param_action::<actions::Save>(cx, Keyword::Save);
    bind_cmd_param_action::<actions::Num0>(cx, 0);
    bind_cmd_param_action::<actions::Num1>(cx, 1);
    bind_cmd_param_action::<actions::Num2>(cx, 2);
    bind_cmd_param_action::<actions::Num3>(cx, 3);
    bind_cmd_param_action::<actions::Num4>(cx, 4);
    bind_cmd_param_action::<actions::Num5>(cx, 5);
    bind_cmd_param_action::<actions::Num6>(cx, 6);
    bind_cmd_param_action::<actions::Num7>(cx, 7);
    bind_cmd_param_action::<actions::Num8>(cx, 8);
    bind_cmd_param_action::<actions::Num9>(cx, 9);
}

pub fn run(showfile_path: Option<PathBuf>) {
    let engine = Engine::new(showfile_path.as_ref()).expect("failed to create engine");

    Application::new().with_assets(ui::assets::Assets).run(move |cx: &mut App| {
        cx.activate(true);

        ui::init(cx).expect("failed to initialize ui crate");
        state::init(engine, cx);

        init(cx);

        MainWindow::open(cx).expect("failed to open main window");
    });
}
