use dcore::components::*;
use dcore::Point;
use diagrammer_core as dcore;
use svg::{self, Node};
#[derive(Debug, Clone, Default)]
pub struct RenderState {
    id_counter: u64,
}

impl RenderState {
    pub fn generate_id(&mut self, prefix: &str) -> String {
        let next_id = self.id_counter;
        self.id_counter += 1;
        if prefix.is_empty() {
            format!("{}", next_id)
        } else {
            format!("{}-{}", prefix, next_id)
        }
    }
}
pub trait Renderable {
    /// Type of reference that is returned upon rendering this component. For most cases it will either
    /// be a String containing the `id`, or `()`.
    type Ref;

    /// render the component, adding it to the given parent node.
    fn render_to<N: Node>(
        &self,
        parent: &mut N,
        state: &mut RenderState,
    ) -> dcore::Result<Self::Ref>;
}

impl Renderable for Rectangle {
    type Ref = ();

    fn render_to<N: Node>(&self, parent: &mut N, _state: &mut RenderState) -> dcore::Result<()> {
        let mut node = svg::node::element::Rectangle::new();
        node.assign("x", self.bounds.left());
        node.assign("y", self.bounds.top());
        node.assign("width", self.bounds.width());
        node.assign("height", self.bounds.height());
        node.assign("stroke-width", self.width);
        node.assign(
            "stroke",
            format!("rgb({}, {}, {})", self.color.r, self.color.g, self.color.b),
        );
        node.assign("fill", "rgb(255,255,255)");

        parent.append(node);

        Ok(())
    }
}

impl Renderable for Line {
    type Ref = ();

    fn render_to<N: Node>(&self, parent: &mut N, state: &mut RenderState) -> dcore::Result<()> {
        // first we need to render out the 'marker' definition
        // for now, we're going to try to put the marker here, rather in some top-level <defs> tag

        let mut node = svg::node::element::Path::new();
        if let Some(ref arrow) = self.start_arrow {
            let id = arrow.render_to(parent, state)?;
            node.assign("marker-start", format!("url(#{})", id));
        }
        if let Some(ref arrow) = self.end_arrow {
            let id = arrow.render_to(parent, state)?;
            node.assign("marker-end", format!("url(#{})", id));
        }
        // now we create a group, and render the line points
        node.assign("fill", "transparent");
        node.assign("stroke", "black");
        node.assign("stroke-width", format!("{}", self.width));
        node.assign("d", build_path(&self.points));

        parent.append(node);

        Ok(())
    }
}

impl Renderable for ArrowHead {
    type Ref = String;
    fn render_to<N: Node>(
        &self,
        parent: &mut N,
        state: &mut RenderState,
    ) -> diagrammer_core::Result<Self::Ref> {
        let mut marker = svg::node::element::Marker::new();
        let arrow_id = state.generate_id("arrow");
        marker.assign("id", arrow_id.clone());

        match self {
            ArrowHead::Solid => {
                marker.assign("refX", 5);
                marker.assign("refY", 5);
                marker.assign("viewBox", "0 0 10 10");
                marker.assign("orient", "auto-start-reverse");
                marker.append(svg::node::element::Path::new().set("d", "M 0 0 L 10 5 L 0 10 z"));
            }
        }

        parent.append(marker);
        Ok(arrow_id)
    }
}

fn build_path(points: &Vec<Point>) -> String {
    let mut output = String::new();
    if !points.is_empty() {
        let start = points[0];
        output.push_str(&format!("M {} {} ", start.x, start.y));
        for p in points.iter().skip(1) {
            output.push_str(&format!("L {} {} ", p.x, p.y));
        }
    }

    output
}
