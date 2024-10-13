use std::ops::Deref;

use gpui::{
    hsla, px, AppContext, Global, Hsla, ModelContext, Pixels, SharedString, ViewContext,
    WindowContext,
};

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> &Theme {
        Theme::get_global(self)
    }
}

impl<'a, V> ActiveTheme for ViewContext<'a, V> {
    fn theme(&self) -> &Theme {
        self.deref().theme()
    }
}

impl<'a, V> ActiveTheme for ModelContext<'a, V> {
    fn theme(&self) -> &Theme {
        self.deref().theme()
    }
}

impl<'a> ActiveTheme for WindowContext<'a> {
    fn theme(&self) -> &Theme {
        self.deref().theme()
    }
}

/// Make a [gpui::Hsla] color.
///
/// - h: 0..360.0
/// - s: 0.0..100.0
/// - l: 0.0..100.0
pub fn hsl(h: f32, s: f32, l: f32) -> Hsla {
    hsla(h / 360., s / 100.0, l / 100.0, 1.0)
}

pub trait Colorize {
    fn opacity(&self, opacity: f32) -> Hsla;
    fn divide(&self, divisor: f32) -> Hsla;
    fn invert(&self) -> Hsla;
    fn invert_l(&self) -> Hsla;
    fn lighten(&self, amount: f32) -> Hsla;
    fn darken(&self, amount: f32) -> Hsla;
}

impl Colorize for Hsla {
    /// Returns a new color with the given opacity.
    ///
    /// The opacity is a value between 0.0 and 1.0, where 0.0 is fully transparent and 1.0 is fully opaque.
    fn opacity(&self, factor: f32) -> Hsla {
        Hsla {
            a: self.a * factor.clamp(0.0, 1.0),
            ..*self
        }
    }

    /// Returns a new color with each channel divided by the given divisor.
    ///
    /// The divisor in range of 0.0 .. 1.0
    fn divide(&self, divisor: f32) -> Hsla {
        Hsla {
            a: divisor,
            ..*self
        }
    }

    /// Return inverted color
    fn invert(&self) -> Hsla {
        Hsla {
            h: (self.h + 1.8) % 3.6,
            s: 1.0 - self.s,
            l: 1.0 - self.l,
            a: self.a,
        }
    }

    /// Return inverted lightness
    fn invert_l(&self) -> Hsla {
        Hsla {
            l: 1.0 - self.l,
            ..*self
        }
    }

    /// Return a new color with the lightness increased by the given factor.
    fn lighten(&self, factor: f32) -> Hsla {
        let l = (self.l * 1.0 + factor.clamp(0.0, 1.0)).min(1.0);

        Hsla { l, ..*self }
    }

    /// Return a new color with the darkness increased by the given factor.
    fn darken(&self, factor: f32) -> Hsla {
        let l = (self.l * 1.0 - factor.clamp(0.0, 1.0)).max(0.0);

        Hsla { l, ..*self }
    }
}

pub struct Theme {
    pub font_size: Pixels,
    pub font_family: SharedString,
    pub radius: Pixels,

    pub background: Hsla,
    pub foreground: Hsla,
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub primary_active: Hsla,
    pub secondary: Hsla,
    pub secondary_hover: Hsla,
    pub secondary_active: Hsla,
    pub tertriary: Hsla,
    pub tertriary_hover: Hsla,
    pub tertriary_active: Hsla,
    pub destructive: Hsla,
    pub destructive_hover: Hsla,
    pub destructive_active: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub border: Hsla,
    pub selection: Hsla,

    pub input_height: Pixels,
    pub cursor_width: Pixels,
    pub cursor_blink_interval_ms: u64,
    pub cursor_blink_pause_delay_ms: u64,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // Fonts
            font_size: px(12.0),
            font_family: "IBM Plex Mono".into(),
            radius: px(4.0),

            // Colors
            background: hsl(0.0, 0.0, 2.0),
            foreground: hsl(0., 0., 98.0),
            primary: hsl(0.0, 0.0, 6.0),
            primary_hover: hsl(0.0, 0.0, 6.0).lighten(0.02),
            primary_active: hsl(0.0, 0.0, 6.0).lighten(0.05),
            secondary: hsl(0.0, 0.0, 10.0),
            secondary_hover: hsl(0.0, 0.0, 10.0).lighten(0.02),
            secondary_active: hsl(0.0, 0.0, 10.0).lighten(0.05),
            tertriary: hsl(0.0, 0.0, 18.0),
            tertriary_hover: hsl(0.0, 0.0, 18.0).lighten(0.02),
            tertriary_active: hsl(0.0, 0.0, 18.0).lighten(0.05),
            destructive: hsl(0.0, 62.8, 30.6),
            destructive_hover: hsl(0.0, 62.8, 35.6),
            destructive_active: hsl(0.0, 62.8, 20.6),
            muted: hsl(240.0, 3.7, 15.9),
            muted_foreground: hsl(240.0, 3.7, 15.9).lighten(0.2),
            accent: hsl(44.0, 98.0, 50.0),
            border: hsl(240.0, 3.7, 15.9),
            selection: hsl(194.0, 67.0, 25.0),

            input_height: px(22.0),
            cursor_width: px(2.0),
            cursor_blink_interval_ms: 750,
            cursor_blink_pause_delay_ms: 500,
        }
    }
}

impl Global for Theme {}

impl Theme {
    pub fn get_global(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }
}
