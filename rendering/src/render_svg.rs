use std::path;

use svg::{
    node::element::{Group, Rectangle},
    Document,
};

use crate::{svg_plot::SvgNode, RenderPrimitive};

pub fn flip_y() -> Group {
    Group::new().set("transform", "scale(1, -1)")
}

pub fn make_card(center_x: f64, center_y: f64, half_width: f64) -> Document {
    // My usual art trading card format for my website is 500x700px
    const WIDTH: f64 = 500.0;
    const HEIGHT: f64 = 700.0;
    const ASPECT_RATIO: f64 = WIDTH / HEIGHT;

    let half_height = half_width / ASPECT_RATIO;

    let offset_x = half_width;
    let offset_y = half_height;
    let left = center_x - offset_x;
    let top = -center_y - offset_y;

    let view_box = (left, top, half_width * 2.0, half_height * 2.0);

    let background = Rectangle::new()
        .set("x", left)
        .set("y", top)
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "black")
        .set("stroke", "none");

    Document::new()
        .set("width", 500)
        .set("height", 700)
        .set("viewBox", view_box)
        .add(background)
}

pub struct View<'a>(pub &'a str, pub f64, pub f64, pub f64);

pub fn render_svg<P: AsRef<path::Path>>(
    output_dir: P,
    prefix: &str,
    views: &[View],
    scene: RenderPrimitive,
) -> Result<(), std::io::Error> {
    let SvgNode(root) = SvgNode::from(scene);
    let flipped = flip_y().add(root);

    for &View(label, x, y, half_width) in views {
        let doc = make_card(x, y, half_width).add(flipped.clone());
        let separator = if label.is_empty() { "" } else { "_" };
        let filename = format!("{}{}{}.svg", prefix, separator, label);
        let path = output_dir.as_ref().join(path::Path::new(&filename));
        svg::save(path, &doc)?
    }

    Ok(())
}
