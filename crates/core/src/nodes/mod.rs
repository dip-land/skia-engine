//! Properties and shared layout options for renderable element nodes.
//!
//! Each node module provides a `Props` struct and a renderer. The shared
//! enums describe placement, alignment, and text overflow behavior.
pub mod container;
pub mod image;
pub mod rect;
pub mod text;

/// Positioning mode used by node renderers.
#[derive(Debug, Clone, Copy)]
pub enum Position {
    /// Position the element using its absolute coordinates.
    Absolute,
    /// Position the element relative to the parent container's origin.
    Relative,
}

/// Vertical alignment of an element inside its parent container.
#[derive(Debug, Clone, Copy)]
pub enum VerticalAlignment {
    /// Align the element's top edge with the parent's top edge.
    Top,
    /// Center the element vertically in the parent.
    Center,
    /// Align the element's bottom edge with the parent's bottom edge.
    Bottom,
}

/// Horizontal alignment of an element inside its parent container.
#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlignment {
    /// Align the element's left edge with the parent's left edge.
    Left,
    /// Center the element horizontally in the parent.
    Center,
    /// Align the element's right edge with the parent's right edge.
    Right,
}

/// Vertical placement of text within its text layout box.
#[derive(Debug, Clone, Copy)]
pub enum TextAlignment {
    /// Place text using its fonts baseline.
    Baseline,
    /// Place the paragraph at the top of its layout box.
    Top,
    /// Center the paragraph vertically in its layout box.
    Middle,
    /// Place the paragraph at the bottom of its layout box.
    Bottom,
}

/// Behavior for text that exceeds its layout bounds.
#[derive(Debug, Clone, Copy)]
pub enum TextOverflow {
    /// Draw text without clipping it to the layout bounds.
    Visible,
    /// Clip text at the layout bounds.
    Clip,
    /// Clip overflowing text and show an ellipsis where supported.
    Ellipsis,
}

/// Re-export of Skia's image sampling configuration for image nodes.
pub use skia_safe::SamplingOptions;
