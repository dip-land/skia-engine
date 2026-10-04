use crate::{ElementNode, nodes};
use skia_safe::Canvas;

pub fn render_tree(canvas: &Canvas, parent_node: Option<&ElementNode>, node: &ElementNode) {
    match node {
        ElementNode::Container { props, children } => {
            nodes::container::render(canvas, parent_node, node, props, children);
        }
        ElementNode::Image { props } => {
            nodes::image::render(canvas, parent_node, props);
        }
        ElementNode::Rect { props } => {
            nodes::rect::render(canvas, parent_node, props);
        }
        ElementNode::Text { props, content } => {
            nodes::text::render(canvas, parent_node, props, content);
        }
    }
}
