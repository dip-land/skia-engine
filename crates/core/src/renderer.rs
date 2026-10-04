use crate::{ElementNode, nodes};
use skia_safe::Canvas;

/// Renders one element and, for containers, recursively renders its children.
///
/// Call this with `parent_node` set to `None` for the root element. Container
/// rendering passes itself as the parent when rendering each child, allowing
/// child positioning and alignment to use the parent container's bounds.
///
/// # Arguments
///
/// * `canvas` - Skia canvas to draw into.
/// * `parent_node` - Parent element, or `None` for the root.
/// * `node` - Element to render.
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
