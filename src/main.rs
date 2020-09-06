use diagrammer_core as dcore;
use diagrammer_render as render;

use dcore::*;
use render::Renderable;

fn main() -> dcore::Result<()> {
    let mut doc = svg::Document::new();

    let r = components::Rectangle {
        bounds: Rect::from_size(10.0, 10.0, 100.0, 50.0),
        color: rgb::RGB8 { r: 0, g: 0, b: 0 },
        width: 10.0.into(),
        ..Default::default()
    };

    let arr = components::Line {
        points: vec![
            Point::new(10.0, 10.0),
            Point::new(50.0, 20.0),
            Point::new(20.0, 30.0),
        ],
        width: Length(1.0, Unit::Raw),
        start_arrow: Some(components::ArrowHead::Solid),
        end_arrow: Some(components::ArrowHead::Solid),
        ..Default::default()
    };

    let mut state = render::RenderState::default();

    // r.render_to(&mut doc, &mut state)?;
    arr.render_to(&mut doc, &mut state)?;

    svg::write(std::io::stdout(), &doc)?;
    Ok(())
}
