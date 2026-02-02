use gpui::{App, Global, Hsla, Pixels, ReadGlobal, px, rgb};

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

            bg_primary: rgb(0x100f0f).into(),
            bg_secondary: rgb(0x1c1b1a).into(),
            bg_tertiary: rgb(0x23262b).into(),

            fg_primary: rgb(0xebebeb).into(),
            fg_secondary: rgb(0xb3b3b3).into(),
            fg_tertiary: rgb(0x808080).into(),

            accent: rgb(0x3bb2f6).into(),
            warning: rgb(0xffc94d).into(),
            error: rgb(0xed2320).into(),
            success: rgb(0x3bb273).into(),

            title_bar: rgb(0x1c1b1a).into(),
            title_bar_border: rgb(0x222120).into(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Global for Theme {}
