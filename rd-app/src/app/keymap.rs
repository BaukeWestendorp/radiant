use rd_ui::{Keymap, KeymapBinding};

pub fn default_keymap() -> Keymap {
    Keymap::new(vec![
        KeymapBinding::new("SettingsOpen", "secondary-,", "Root"),
        KeymapBinding::new("cmd::Save", "secondary-s", "Root"),
        KeymapBinding::new("cmd::Highlight", "h", "!TextInput"),
    ])
}
