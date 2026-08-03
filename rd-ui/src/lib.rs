mod asset;
mod binding;
mod button;

mod app;
mod container;
mod grid;
mod icon;
mod input;
mod keymap;
mod org;
mod popup;
mod root;
mod scrollable;
mod settings;
mod table;
mod tabs;
mod theme;
mod tiles;
mod title_bar;
mod typo;
mod util;

pub use app::{AppBuilder, build_app};
pub use asset::Assets;
pub use binding::Binding;
pub use button::{Button, ButtonVariant};
pub use container::{ContainerStyle, container, interactive_container};
pub use grid::{dot_grid, line_grid, scrollable_line_grid};
pub use icon::{Icon, IconSize, IconVariant};
pub use input::{
    AutoInput, Field, FieldValue, Form, FormDelegate, FormField, FormFocusBehavior, INPUT_HEIGHT,
    Input, InputDelegate, InputEvent, InputState, Labelled, LayoutDirection, OptionInput, Picker,
    PickerKind, Slider, SliderValue, TextInput,
};
pub use keymap::{Keymap, KeymapBinding};
pub use org::section;
pub use popup::{Popup, PopupAppExt};
pub use root::Root;
pub use scrollable::{Scrollable, ScrollableState};
pub use settings::{SETTINGS_WINDOW_OPTIONS, SettingsAppExt};
pub use table::{
    Column, EnumerableValue, Table, TableDelegate, TableEvent, TableSelection, TableState,
};
pub use tabs::{Tab, Tabs, TabsState, TabsVariant};
pub use theme::{ActiveTheme, HslaExt};
pub use tiles::{PoolTile, PoolTileDelegate, TileDelegate, TileGrid, TileGridState};
pub use title_bar::{TITLE_BAR_HEIGHT, TitleBar};
pub use typo::{article, h1, h2, h3, h4, h5, h6, link, sub};
pub use util::{
    FocusableExt, StatefulInteractiveElementExt, StyledExt, StyledParentExt, h_flex, todo, v_flex,
    z_stack,
};

pub use ::gpui;

#[cfg(feature = "derive")]
pub use rd_ui_derive::*;

pub fn init(cx: &mut gpui::App) {
    crate::theme::init(cx);
    crate::popup::init(cx);
    crate::settings::init(cx);
    crate::app::action::init(cx);
}

pub mod action {
    gpui::actions!([Edit, Delete, ClearSelection, SelectAll]);
}
