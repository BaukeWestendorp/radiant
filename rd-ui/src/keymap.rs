use gpui::{App, KeyBindingContextPredicate};

pub struct Keymap {
    bindings: Vec<KeymapBinding>,
}

impl Keymap {
    pub fn new(bindings: Vec<KeymapBinding>) -> Self {
        Self { bindings }
    }

    pub fn apply(&self, cx: &mut App) {
        let key_bindings = self
            .bindings
            .iter()
            .flat_map(|binding| match cx.build_action(&binding.action_name, None) {
                Ok(action) => {
                    let context = if binding.context.is_empty() {
                        None
                    } else {
                        KeyBindingContextPredicate::parse(&binding.context).ok().map(|p| p.into())
                    };

                    match gpui::KeyBinding::load(
                        &binding.keystrokes,
                        action,
                        context,
                        false,
                        None,
                        cx.keyboard_mapper().as_ref(),
                    ) {
                        Ok(key_binding) => Some(key_binding),
                        Err(err) => {
                            log::warn!("Invalid keystroke: {}", err.keystroke);
                            None
                        }
                    }
                }
                Err(err) => {
                    log::warn!("Failed to generate Action for {}: {err}", binding.action_name);
                    None
                }
            })
            .collect::<Vec<_>>();

        cx.bind_keys(key_bindings);
    }
}

// ... rest of your code ...

pub struct KeymapBinding {
    action_name: String,
    keystrokes: String,
    context: String,
}

impl KeymapBinding {
    pub fn new(
        action_name: impl Into<String>,
        keystrokes: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self {
            action_name: action_name.into(),
            keystrokes: keystrokes.into(),
            context: context.into(),
        }
    }
}

#[rustfmt::skip]
pub fn default_keymap() -> Keymap {
    Keymap::new(vec![
        KeymapBinding::new("Quit", "secondary-q", "Root"),
        KeymapBinding::new("Tab", "tab", "Root"),
        KeymapBinding::new("TabPrev", "shift-tab", "Root"),

        KeymapBinding::new("text_input::SelectAll", "secondary-a", "TextInput"),
        KeymapBinding::new("text_input::MoveRight", "right", "TextInput"),
        KeymapBinding::new("text_input::Backspace", "backspace", "TextInput"),
        KeymapBinding::new("text_input::MoveLeft", "left", "TextInput"),
        KeymapBinding::new("text_input::Submit", "enter", "TextInput"),
        KeymapBinding::new("text_input::Delete", "delete", "TextInput"),
        KeymapBinding::new("text_input::Paste", "secondary-v", "TextInput"),
        KeymapBinding::new("text_input::Copy", "secondary-c", "TextInput"),
        KeymapBinding::new("text_input::Cut", "secondary-x", "TextInput"),
        KeymapBinding::new("text_input::SelectRight", "shift-right", "TextInput"),
        KeymapBinding::new("text_input::SelectLeft", "shift-left", "TextInput"),

        #[cfg(target_os = "macos")]      KeymapBinding::new("text_input::MoveToStartOfLine", "cmd-left", "TextInput"),
        #[cfg(not(target_os = "macos"))] KeymapBinding::new("text_input::MoveToStartOfLine", "home", "TextInput"),
        #[cfg(target_os = "macos")]      KeymapBinding::new("text_input::MoveToEndOfLine", "cmd-right", "TextInput"),
        #[cfg(not(target_os = "macos"))] KeymapBinding::new("text_input::MoveToEndOfLine", "end", "TextInput"),
        #[cfg(target_os = "macos")]      KeymapBinding::new("text_input::SelectToStartOfLine", "shift-cmd-left", "TextInput"),
        #[cfg(not(target_os = "macos"))] KeymapBinding::new("text_input::SelectToStartOfLine", "shift-home", "TextInput"),
        #[cfg(target_os = "macos")]      KeymapBinding::new("text_input::SelectToEndOfLine", "shift-cmd-right", "TextInput"),
        #[cfg(not(target_os = "macos"))] KeymapBinding::new("text_input::SelectToEndOfLine", "shift-end", "TextInput"),
        #[cfg(target_os = "macos")]      KeymapBinding::new("text_input::MoveToPreviousWord", "alt-left", "TextInput"),
        #[cfg(not(target_os = "macos"))] KeymapBinding::new("text_input::MoveToPreviousWord", "ctrl-left", "TextInput"),
        #[cfg(target_os = "macos")]      KeymapBinding::new("text_input::MoveToNextWord", "alt-right", "TextInput"),
        #[cfg(not(target_os = "macos"))] KeymapBinding::new("text_input::MoveToNextWord", "ctrl-right", "TextInput"),
        #[cfg(target_os = "macos")]      KeymapBinding::new("text_input::SelectToStartOfWord", "alt-shift-left", "TextInput"),
        #[cfg(not(target_os = "macos"))] KeymapBinding::new("text_input::SelectToStartOfWord", "shift-ctrl-left", "TextInput"),
        #[cfg(target_os = "macos")]      KeymapBinding::new("text_input::SelectToEndOfWord", "alt-shift-right", "TextInput"),
        #[cfg(not(target_os = "macos"))] KeymapBinding::new("text_input::SelectToEndOfWord", "shift-ctrl-right", "TextInput"),

        KeymapBinding::new("table::SelectAll", "secondary-a", "Table"),
        KeymapBinding::new("table::PrevRow", "up", "Table"),
        KeymapBinding::new("table::NextRow", "down", "Table"),
        KeymapBinding::new("table::ToggleExpandSelection", "tab", "Table"),
        KeymapBinding::new("table::ExtendSelectionPrev", "secondary-up", "Table"),
        KeymapBinding::new("table::ExtendSelectionNext", "secondary-down", "Table"),
        KeymapBinding::new("table::DeleteSelection", "delete", "Table"),
        KeymapBinding::new("table::ClearSelection", "escape", "Table"),
        KeymapBinding::new("table::EditSelection", "enter", "Table"),
        KeymapBinding::new("table::PrevColumn", "left", "Table"),
        KeymapBinding::new("table::NextColumn", "right", "Table"),
    ])
}
