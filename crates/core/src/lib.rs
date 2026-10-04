//! Core element types and Skia rendering for `skia-engine`.
//!
//! Build an [`ElementNode`] tree from the node property types in [`nodes`],
//! create a Skia canvas, and draw the tree with [`renderer::render_tree`].
pub mod nodes;
pub mod renderer;

/// A renderable UI element.
///
/// Containers own child elements; image, rectangle, and text nodes are leaves.
pub enum ElementNode {
    /// A layout container that can draw a background and render child elements.
    Container {
        /// Size, position, alignment, and appearance of the container.
        props: nodes::container::Props,
        /// Child elements, rendered in vector order.
        children: Vec<ElementNode>,
    },
    /// An image loaded from a local file or, when an HTTP client feature is
    /// enabled, a URL.
    Image {
        /// Size, position, source, sampling, and appearance of the image.
        props: nodes::image::Props,
    },
    /// A filled rectangle.
    Rect {
        /// Size, position, alignment, fill color, and corner radius.
        props: nodes::rect::Props,
    },
    /// Text rendered from a string.
    Text {
        /// Font, layout, position, alignment, color, and overflow settings.
        props: nodes::text::Props,
        /// Text content rendered by the node.
        content: &'static str,
    },
}
