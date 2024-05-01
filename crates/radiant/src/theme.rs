use gpui::{rgb, Hsla};
use lazy_static::lazy_static;

pub struct Theme {
    pub text: Hsla,
    pub text_secondary: Hsla,
    pub text_tertiary: Hsla,
    pub text_placeholder: Hsla,

    pub background: Hsla,
    pub background_secondary: Hsla,
    pub background_tertiary: Hsla,

    pub fill: Hsla,
    pub fill_secondary: Hsla,
    pub fill_tertiary: Hsla,
    pub fill_selected: Hsla,

    pub border: Hsla,
    pub border_secondary: Hsla,
    pub border_tertiary: Hsla,
    pub border_selected: Hsla,

    pub header: Hsla,
    pub header_border: Hsla,

    pub accent: Hsla,

    pub status_complete_selection: Hsla,
    pub status_programmer_change: Hsla,
}

lazy_static! {
    pub static ref THEME: Theme = Theme {
        text: rgb(0xffffff).into(),
        text_secondary: rgb(0xaaaaaa).into(),
        text_tertiary: rgb(0x808080).into(),
        text_placeholder: rgb(0xb8b8b8).into(),

        background: rgb(0x000000).into(),
        background_secondary: rgb(0x0a0a0a).into(),
        background_tertiary: rgb(0x202020).into(),

        fill: rgb(0x000000).into(),
        fill_secondary: rgb(0x1b1b1b).into(),
        fill_tertiary: rgb(0x282828).into(),
        fill_selected: rgb(0x8d6800).into(),

        border: rgb(0x505050).into(),
        border_secondary: rgb(0x303030).into(),
        border_tertiary: rgb(0x181818).into(),
        border_selected: rgb(0xe9ad00).into(),

        header: rgb(0x1010b0).into(),
        header_border: rgb(0x0000ff).into(),

        accent: rgb(0xe9ad00).into(),

        status_complete_selection: rgb(0x8fff00).into(),
        status_programmer_change: rgb(0xff0808).into(),
    };
}

pub trait Hoverable {
    fn hovered(self) -> Self;
}

impl Hoverable for Hsla {
    fn hovered(self) -> Self {
        let mut hsla = self;
        hsla.l = (hsla.l + 0.05).clamp(0.0, 1.0);
        hsla
    }
}

pub trait Disableable {
    fn disabled(self) -> Self;
}

impl Disableable for Hsla {
    fn disabled(self) -> Self {
        let mut hsla = self;
        hsla.a = (hsla.a - 0.4).clamp(0.0, 1.0);
        hsla
    }
}

pub trait Activatable {
    fn active(self) -> Self;
}

impl Activatable for Hsla {
    fn active(self) -> Self {
        let mut hsla = self;
        hsla.l = (hsla.l + 0.1).clamp(0.0, 1.0);
        hsla
    }
}
