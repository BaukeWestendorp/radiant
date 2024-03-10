use gpui::Rgba;

pub fn darken(color: Rgba, factor: f32) -> Rgba {
    Rgba {
        r: color.r * (1.0 - factor),
        g: color.g * (1.0 - factor),
        b: color.b * (1.0 - factor),
        a: color.a,
    }
}

pub fn lighten(color: Rgba, factor: f32) -> Rgba {
    Rgba {
        r: color.r * factor,
        g: color.g * factor,
        b: color.b * factor,
        a: color.a,
    }
}

pub fn opacify(color: Rgba, factor: f32) -> Rgba {
    Rgba {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a * factor,
    }
}
