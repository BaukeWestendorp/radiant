mod button;
mod root;
mod settings;
mod table;
mod tabs;
mod theme;
mod title_bar;

mod app_ext;
mod element_ext;
mod styled_ext;

pub use button::Button;
pub use root::Root;
pub use table::{Column, Table, TableDelegate, TableState};
pub use tabs::{Tab, Tabs, TabsState, TabsVariant};
pub use theme::ActiveTheme;
pub use title_bar::TitleBar;

pub use app_ext::AppExt;
pub use element_ext::ElementExt;
pub use styled_ext::{StyledExt, h_flex, v_flex};

pub fn init(cx: &mut gpui::App) {
    root::action::init(cx);
    theme::init(cx);
    settings::init(cx);
    table::action::init(cx);
}
