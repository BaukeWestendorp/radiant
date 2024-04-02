use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub struct Bounds {
    pub size: Size,
    pub origin: Point,
}

#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub struct Size {
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub struct Point {
    x: usize,
    y: usize,
}
