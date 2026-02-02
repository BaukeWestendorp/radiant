use gpui::{App, Global, Hsla, Pixels, ReadGlobal, px, rgba};

pub(crate) fn init(cx: &mut App) {
    cx.set_global(Theme::default());
}

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    #[inline(always)]
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub font_size: Pixels,
    pub radius: Pixels,
    pub shadow: bool,

    pub bg_primary: Hsla,
    pub bg_secondary: Hsla,
    pub bg_tertiary: Hsla,

    pub fg_primary: Hsla,
    pub fg_secondary: Hsla,
    pub fg_tertiary: Hsla,

    pub accent: Hsla,
    pub warning: Hsla,
    pub error: Hsla,
    pub success: Hsla,

    pub title_bar: Hsla,
    pub title_bar_border: Hsla,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            font_size: px(16.0),
            radius: px(6.0),
            shadow: true,

            bg_primary: rgba(0x1d1f22ff).into(),
            bg_secondary: rgba(0x23262bff).into(),
            bg_tertiary: rgba(0x2a2e34ff).into(),

            fg_primary: rgba(0xebebebff).into(),
            fg_secondary: rgba(0xb3b3b3ff).into(),
            fg_tertiary: rgba(0x808080ff).into(),

            accent: rgba(0x3bb2f6ff).into(),
            warning: rgba(0xffc94dff).into(),
            error: rgba(0xf25f5cff).into(),
            success: rgba(0x3bb273ff).into(),

            title_bar: rgba(0x23262bff).into(),
            title_bar_border: rgba(0x191b1fff).into(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Global for Theme {}
