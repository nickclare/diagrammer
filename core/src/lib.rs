use std::ops::{Add, Mul, Sub};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("rendering failed: {reason}")]
    Render { reason: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unknown error: {reason}")]
    Unknown { reason: String },
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Default)]
pub struct Point<T = f32> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

#[derive(Default, Debug)]
pub struct Rect<T = f32> {
    pub top_left: Point<T>,
    pub bottom_right: Point<T>,
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    pub fn new(top_left: Point<T>, bottom_right: Point<T>) -> Self {
        Rect {
            top_left,
            bottom_right,
        }
    }

    pub fn from_size(x: T, y: T, width: T, height: T) -> Self {
        let top_left = Point::new(x, y);
        let bottom_right = Point::new(x + width, y + height);
        Rect {
            top_left,
            bottom_right,
        }
    }

    pub fn top(&self) -> T {
        self.top_left.y
    }

    pub fn left(&self) -> T {
        self.top_left.x
    }

    pub fn bottom(&self) -> T {
        self.bottom_right.y
    }

    pub fn right(&self) -> T {
        self.bottom_right.x
    }

    pub fn width(&self) -> T {
        self.right() - self.left()
    }

    pub fn height(&self) -> T {
        self.bottom() - self.top()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc, //percent

    Raw,
}

#[derive(Clone, Copy, Debug)]
pub struct Length(pub f32, pub Unit);

impl Default for Length {
    fn default() -> Self {
        Length(0.0, Unit::Raw)
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Unit::*;
        let suffix = match self {
            Em => "em",
            Ex => "ex",
            Px => "px",
            In => "in",
            Cm => "cm",
            Mm => "mm",
            Pt => "pt",
            Pc => "%",
            Raw => "",
        };
        write!(f, "{}", suffix)
    }
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{1:.*}{2}", precision, self.0, self.1)
        } else {
            write!(f, "{}{}", self.0, self.1)
        }
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Length(value, Unit::Raw)
    }
}

use svg::node::Value;

impl Into<Value> for Length {
    fn into(self) -> Value {
        Value::from(format!("{}", self))
    }
}

pub mod components;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_formatting() {
        let raw = Length::from(10.0);

        let simple = format!("{}", raw);
        assert_eq!(simple, "10");
        let decimals = format!("{:.2}", raw);
        assert_eq!(decimals, "10.00");

        let pixels = Length(10.0, Unit::Px);
        let simple = format!("{}", pixels);
        assert_eq!(simple, "10px");
        let decimals = format!("{:.2}", pixels);
        assert_eq!(decimals, "10.00px");
    }
}
