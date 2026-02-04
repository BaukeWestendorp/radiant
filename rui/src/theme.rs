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

pub trait HslaExt {
    /// Returns a disabled variant of the color (lower alpha and desaturated).
    fn disabled(&self) -> Hsla;

    /// Returns a hover variant of the color (slightly lighter).
    fn hover(&self) -> Hsla;

    /// Returns an active variant of the color (lighter).
    fn active(&self) -> Hsla;
}

impl HslaExt for Hsla {
    fn disabled(&self) -> Hsla {
        let mut c = *self;
        c.s *= 0.4;
        c.a *= 0.5;
        c
    }

    fn hover(&self) -> Hsla {
        let mut c = *self;
        c.l = (c.l + 0.08).min(1.0);
        c
    }

    fn active(&self) -> Hsla {
        let mut c = *self;
        c.l = (c.l + 0.15).min(1.0);
        c
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
    pub bg_selected: Hsla,
    pub bg_table: Hsla,
    pub bg_table_odd: Hsla,

    pub fg_primary: Hsla,
    pub fg_secondary: Hsla,
    pub fg_tertiary: Hsla,
    pub fg_selected: Hsla,

    pub border_primary: Hsla,
    pub border_secondary: Hsla,
    pub border_tertiary: Hsla,
    pub border_selected: Hsla,

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
            radius: px(3.0),
            shadow: true,

            bg_primary: rgb(0x100f0f).into(),
            bg_secondary: rgb(0x1c1b1a).into(),
            bg_tertiary: rgb(0x302e2d).into(),
            bg_selected: rgb(0x232a36).into(),
            bg_table: rgb(0x100f0f).into(),
            bg_table_odd: rgb(0x151414).into(),

            fg_primary: rgb(0xebebeb).into(),
            fg_secondary: rgb(0xb3b3b3).into(),
            fg_tertiary: rgb(0x808080).into(),
            fg_selected: rgb(0xffffff).into(),

            border_primary: rgb(0x292929).into(),
            border_secondary: rgb(0x353535).into(),
            border_tertiary: rgb(0x404040).into(),
            border_selected: rgb(0x3bb2f6).into(),

            accent: rgb(0x3bb2f6).into(),
            warning: rgb(0xffc94d).into(),
            error: rgb(0xed2320).into(),
            success: rgb(0x3bb273).into(),

            title_bar: rgb(0x1c1b1a).into(),
            title_bar_border: rgb(0x353535).into(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Global for Theme {}
