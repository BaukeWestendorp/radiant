use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub struct Bounds {
    pub size: Size,
    pub origin: Point,
}

impl Bounds {
    pub(crate) fn area(&self) -> usize {
        self.size.width * self.size.height
    }
}

#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
