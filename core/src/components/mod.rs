use super::*;
use rgb::RGB8;

/// A basic rectangle with a stroke
/// todo: add fill, dashed edges, etc.
#[derive(Default, Debug)]
pub struct Rectangle {
    pub bounds: Rect,
    pub color: RGB8,
    pub width: Length,
}

#[derive(Debug, Default)]
pub struct Text {
    pub position: Point,
    pub text: String,
    pub font_size: Length, //check this
                           // todo support other fonts
}

#[derive(Debug, Default)]
pub struct Line {
    pub points: Vec<Point>,
    pub width: Length,
    // todo: dash patterns, arrow head types, etc.
    pub start_arrow: Option<ArrowHead>,
    pub end_arrow: Option<ArrowHead>,
}

#[derive(Debug)]
pub enum ArrowHead {
    Solid,
}
