/// A point in 2D space.
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Default, serde::Serialize, serde::Deserialize,
)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(feature = "gpui")]
impl From<gpui::Point<gpui::Pixels>> for Point {
    fn from(value: gpui::Point<gpui::Pixels>) -> Self {
        Self {
            x: value.x.0,
            y: value.y.0,
        }
    }
}

#[cfg(feature = "gpui")]
impl From<Point> for gpui::Point<gpui::Pixels> {
    fn from(value: Point) -> Self {
        Self::new(value.x.into(), value.y.into())
    }
}
