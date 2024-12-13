#![allow(clippy::type_complexity)]
#![allow(clippy::option_as_ref_deref)]

pub mod button;
pub mod container;
pub mod input;
pub mod styled_ext;
pub mod table;
pub mod theme;
pub mod utils;

pub use button::*;
pub use container::*;
pub use input::*;
pub use styled_ext::*;
pub use table::*;
pub use theme::*;
pub use utils::*;

pub fn init(cx: &mut gpui::AppContext) {
    cx.set_global(Theme::default());
    input::text_field::init(cx);
}
