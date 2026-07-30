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
    fn disabled(&self) -> Hsla;
    fn hover(&self) -> Hsla;
    fn active(&self) -> Hsla;
    fn contrast(&self) -> Hsla;

    fn with_h(&self, h: f32) -> Hsla;
    fn with_s(&self, s: f32) -> Hsla;
    fn with_l(&self, l: f32) -> Hsla;
    fn with_a(&self, a: f32) -> Hsla;
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
        let gamma = 1.8;
        let l = c.l.powf(gamma);
        let l = (l + 0.06).min(1.0);
        c.l = l.powf(1.0 / gamma);
        c
    }

    fn active(&self) -> Hsla {
        let mut c = *self;
        let gamma = 1.8;
        let l = c.l.powf(gamma);
        let l = (l + 0.10).min(1.0);
        c.l = l.powf(1.0 / gamma);
        c
    }

    fn contrast(&self) -> Hsla {
        let mut c = *self;
        c.l = if c.l > 0.5 { 0.1 } else { 0.9 };
        c
    }

    fn with_h(&self, h: f32) -> Hsla {
        let mut c = *self;
        c.h = h;
        c
    }

    fn with_s(&self, s: f32) -> Hsla {
        let mut c = *self;
        c.s = s;
        c
    }

    fn with_l(&self, l: f32) -> Hsla {
        let mut c = *self;
        c.l = l;
        c
    }

    fn with_a(&self, a: f32) -> Hsla {
        let mut c = *self;
        c.a = a;
        c
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub font_size: Pixels,
    pub radius: Pixels,
    pub shadow: bool,
    pub cursor_width: Pixels,

    pub bg_primary: Hsla,
    pub bg_secondary: Hsla,
    pub bg_tertiary: Hsla,
    pub bg_selected: Hsla,
    pub bg_focus: Hsla,
    pub bg_table: Hsla,
    pub bg_table_odd: Hsla,
    pub bg_tile_header: Hsla,

    pub fg_primary: Hsla,
    pub fg_secondary: Hsla,
    pub fg_tertiary: Hsla,
    pub fg_selected: Hsla,
    pub fg_focus: Hsla,
    pub fg_tile_header: Hsla,

    pub border_primary: Hsla,
    pub border_secondary: Hsla,
    pub border_tertiary: Hsla,
    pub border_selected: Hsla,
    pub border_focus: Hsla,
    pub border_tile_header: Hsla,

    pub accent: Hsla,
    pub indicate: IndicationColors,

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
        let accent: Hsla = rgb(0xee5622).into();

        Self {
            font_size: px(14.0),
            radius: px(3.0),
            shadow: true,
            cursor_width: px(2.0),

            bg_primary: rgb(0xffffff).into(),
            bg_secondary: rgb(0xf4f4f4).into(),
            bg_tertiary: rgb(0xeaeaea).into(),
            bg_selected: accent.with_s(0.708).with_l(0.89),
            bg_focus: accent.with_s(0.608).with_l(0.95),
            bg_table: rgb(0xffffff).into(),
            bg_table_odd: rgb(0xf9f9f8).into(),
            bg_tile_header: accent.with_s(0.386).with_l(0.8),

            fg_primary: hsla(0., 0., 0.07, 1.).into(),
            fg_secondary: hsla(0., 0., 0.3, 1.).into(),
            fg_tertiary: rgb(0x808080).into(),
            fg_selected: accent.with_s(0.4).with_l(0.1),
            fg_focus: accent.with_s(0.912).with_l(0.15),
            fg_tile_header: accent.with_s(0.667).with_l(0.1),

            border_primary: hsla(0., 0., 0.84, 1.).into(),
            border_secondary: hsla(0., 0., 0.8, 1.).into(),
            border_tertiary: hsla(0., 0., 0.75, 1.).into(),
            border_selected: accent.with_s(0.912).with_l(0.4),
            border_focus: accent.with_s(0.912).with_l(0.5),
            border_tile_header: accent.with_s(0.386).with_l(0.725),

            accent,
            indicate: IndicationColors::light(accent),

            contrast: rgb(0x000000).into(),

            title_bar: hsla(0.083, 0.037, 0.894, 1.).into(),
            title_bar_border: hsla(0., 0., 0.8, 1.).into(),

            button_depression: px(1.0),
        }
    }

    pub fn dark() -> Self {
        let accent: Hsla = rgb(0xee5622).into();

        Self {
            font_size: px(14.0),
            radius: px(3.0),
            shadow: true,
            cursor_width: px(2.0),

            bg_primary: rgb(0x100f0f).into(),
            bg_secondary: rgb(0x1c1b1a).into(),
            bg_tertiary: rgb(0x302e2d).into(),
            bg_selected: accent.with_s(0.513).with_l(0.275),
            bg_focus: accent.with_s(0.55).with_l(0.18),
            bg_table: rgb(0x100f0f).into(),
            bg_table_odd: rgb(0x151414).into(),
            bg_tile_header: accent.with_s(0.38).with_l(0.20),

            fg_primary: rgb(0xebebeb).into(),
            fg_secondary: rgb(0xb3b3b3).into(),
            fg_tertiary: rgb(0x808080).into(),
            fg_selected: accent.with_s(0.90).with_l(0.86),
            fg_focus: accent.with_s(0.90).with_l(0.86),
            fg_tile_header: accent.with_s(0.61).with_l(0.92),

            border_primary: rgb(0x292929).into(),
            border_secondary: rgb(0x353535).into(),
            border_tertiary: rgb(0x404040).into(),
            border_selected: accent,
            border_focus: accent,
            border_tile_header: accent.with_s(0.38).with_l(0.27),

            accent,
            indicate: IndicationColors::dark(accent),

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

#[derive(Debug, Clone)]
pub struct IndicationColors {
    pub danger: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
    pub success: Hsla,
}

impl IndicationColors {
    pub fn light(accent: Hsla) -> Self {
        Self {
            danger: rgb(0xe12e2c).into(),
            warning: rgb(0xffc94d).into(),
            info: rgb(0x3bb2f6).into(),
            success: rgb(0x9ce152).into(),
        }
    }

    pub fn dark(accent: Hsla) -> Self {
        Self {
            danger: rgb(0xe12e2c).into(),
            warning: rgb(0xffc94d).into(),
            info: rgb(0x3bb2f6).into(),
            success: rgb(0x9ce152).into(),
        }
    }
}
