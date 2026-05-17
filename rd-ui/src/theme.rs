use gpui::{App, Global, Hsla, Pixels, ReadGlobal, WindowAppearance, hsla, px, rgb};

pub(crate) fn init(cx: &mut App) {
    // FIXME: Theme does not change when the system appearance changes.
    cx.set_global(Theme::system(cx));
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
    pub bg_selected_extra: Hsla,
    pub bg_table: Hsla,
    pub bg_table_odd: Hsla,
    pub bg_tile_header: Hsla,

    pub fg_primary: Hsla,
    pub fg_secondary: Hsla,
    pub fg_tertiary: Hsla,
    pub fg_selected: Hsla,
    pub fg_tile_header: Hsla,

    pub border_primary: Hsla,
    pub border_secondary: Hsla,
    pub border_tertiary: Hsla,
    pub border_selected: Hsla,
    pub border_tile_header: Hsla,

    pub accent: Hsla,
    pub warning: Hsla,
    pub error: Hsla,
    pub success: Hsla,

    pub contrast: Hsla,

    pub title_bar: Hsla,
    pub title_bar_border: Hsla,

    pub button_depression: Pixels,
}

impl Theme {
    pub fn system(cx: &App) -> Self {
        match cx.window_appearance() {
            WindowAppearance::Light => Self::light(),
            WindowAppearance::VibrantLight => Self::light(),
            WindowAppearance::Dark => Self::dark(),
            WindowAppearance::VibrantDark => Self::dark(),
        }
    }

    pub fn light() -> Self {
        Self {
            font_size: px(14.0),
            radius: px(3.0),
            shadow: true,

            bg_primary: rgb(0xffffff).into(),
            bg_secondary: rgb(0xf4f4f4).into(),
            bg_tertiary: rgb(0xeaeaea).into(),
            bg_selected: hsla(0.6, 0.508, 0.89, 1.).into(),
            bg_selected_extra: hsla(0.605, 0.213, 0.76, 1.).into(),
            bg_table: rgb(0xffffff).into(),
            bg_table_odd: rgb(0xf9f9f8).into(),
            bg_tile_header: hsla(0.577, 0.386, 0.8, 1.).into(),

            fg_primary: hsla(0., 0., 0.07, 1.).into(),
            fg_secondary: hsla(0., 0., 0.3, 1.).into(),
            fg_tertiary: rgb(0x808080).into(),
            fg_selected: hsla(0.562, 0.912, 0.15, 1.).into(),
            fg_tile_header: hsla(0.554, 0.667, 0.1, 1.).into(),

            border_primary: hsla(0., 0., 0.84, 1.).into(),
            border_secondary: hsla(0., 0., 0.8, 1.).into(),
            border_tertiary: hsla(0., 0., 0.75, 1.).into(),
            border_selected: hsla(0.561, 0.912, 0.4, 1.).into(),
            border_tile_header: hsla(0.571, 0.386, 0.725, 1.).into(),

            accent: rgb(0x3bb2f6).into(),
            warning: rgb(0xffc94d).into(),
            error: rgb(0xed2320).into(),
            success: rgb(0x3bb273).into(),

            contrast: rgb(0x000000).into(),

            title_bar: hsla(0.083, 0.037, 0.894, 1.).into(),
            title_bar_border: hsla(0., 0., 0.8, 1.).into(),

            button_depression: px(1.0),
        }
    }

    pub fn dark() -> Self {
        Self {
            font_size: px(14.0),
            radius: px(3.0),
            shadow: true,

            bg_primary: rgb(0x100f0f).into(),
            bg_secondary: rgb(0x1c1b1a).into(),
            bg_tertiary: rgb(0x302e2d).into(),
            bg_selected: rgb(0x232a36).into(),
            bg_selected_extra: rgb(0x556683).into(),
            bg_table: rgb(0x100f0f).into(),
            bg_table_odd: rgb(0x151414).into(),
            bg_tile_header: rgb(0x1f3446).into(),

            fg_primary: rgb(0xebebeb).into(),
            fg_secondary: rgb(0xb3b3b3).into(),
            fg_tertiary: rgb(0x808080).into(),
            fg_selected: rgb(0xbee5fc).into(),
            fg_tile_header: rgb(0xdceff8).into(),

            border_primary: rgb(0x292929).into(),
            border_secondary: rgb(0x353535).into(),
            border_tertiary: rgb(0x404040).into(),
            border_selected: rgb(0x3bb2f6).into(),
            border_tile_header: rgb(0x2b4a61).into(),

            accent: rgb(0x3bb2f6).into(),
            warning: rgb(0xffc94d).into(),
            error: rgb(0xed2320).into(),
            success: rgb(0x3bb273).into(),

            contrast: rgb(0xffffff).into(),

            title_bar: rgb(0x1c1b1a).into(),
            title_bar_border: rgb(0x353535).into(),

            button_depression: px(1.0),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Global for Theme {}
