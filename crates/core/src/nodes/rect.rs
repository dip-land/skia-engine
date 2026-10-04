use crate::{
    ElementNode,
    nodes::{HorizontalAlignment, Position, VerticalAlignment},
};
use skia_safe::{Canvas, Color, Paint, Rect as SkiaRect};

/// Properties that control a rectangle's size, placement, and appearance.
pub struct Props {
    /// Rectangle width in pixels.
    pub width: f32,
    /// Rectangle height in pixels.
    pub height: f32,

    /// Positioning mode relative to the parent container.
    pub position: Option<Position>,
    /// Horizontal position in pixels; defaults to `0.0`.
    pub x: Option<f32>,
    /// Vertical position in pixels; defaults to `0.0`.
    pub y: Option<f32>,

    /// Optional vertical alignment within the parent container.
    ///
    /// When set for a child, this takes precedence over `y` and the vertical
    /// positioning mode.
    pub vertical_alignment: Option<VerticalAlignment>,
    /// Optional horizontal alignment within the parent container.
    ///
    /// When set for a child, this takes precedence over `x` and the horizontal
    /// positioning mode.
    pub horizontal_alignment: Option<HorizontalAlignment>,

    /// Fill color of the rectangle.
    pub background_color: Color,
    /// Optional corner radius in pixels. When absent, the rectangle has square
    /// corners.
    pub border_radius: Option<f32>,
}

impl Default for Props {
    /// Creates a 10-by-10 black rectangle at the origin, with absolute
    /// positioning and centered alignment defaults for use as a child.
    fn default() -> Self {
        Self {
            width: 10.0,
            height: 10.0,
            position: Some(Position::Absolute),
            x: Some(0.0),
            y: Some(0.0),
            vertical_alignment: Some(VerticalAlignment::Center),
            horizontal_alignment: Some(HorizontalAlignment::Center),
            background_color: Color::BLACK,
            border_radius: None,
        }
    }
}

/// Draws a rectangle, using a rounded rectangle when `border_radius` is set.
///
/// For a child of a container, relative positioning adds the parent's origin.
/// A configured alignment replaces the computed coordinate on that axis.
/// Root rectangles use their coordinates directly.
///
/// # Arguments
///
/// * `canvas` - Skia canvas to draw into.
/// * `parent_node` - Parent element, if this rectangle is nested in a tree.
/// * `props` - Size, placement, and appearance properties.
pub fn render(canvas: &Canvas, parent_node: Option<&ElementNode>, props: &Props) {
    let (parent_x, parent_y, parent_width, parent_height) = match parent_node {
        Some(ElementNode::Container { props, .. }) => (
            props.x.unwrap_or(0.0),
            props.y.unwrap_or(0.0),
            props.width.unwrap_or(0.0),
            props.height.unwrap_or(0.0),
        ),
        _ => (0.0, 0.0, 0.0, 0.0),
    };
    let mut x = props.x.unwrap_or(0.0);
    let mut y = props.y.unwrap_or(0.0);
    let width = props.width;
    let height = props.height;

    // keep the position relative to the parent if it's a child
    let position = props.position.unwrap_or(Position::Absolute);
    if parent_node.is_some() {
        if let Position::Relative = position {
            x += parent_x;
            y += parent_y;
        }

        // overwrite Y if VerticalAlignment is set
        if let Some(vertical_alignment) = props.vertical_alignment {
            match vertical_alignment {
                VerticalAlignment::Top => y = parent_y,
                VerticalAlignment::Center => y = parent_y + (parent_height - height) / 2.0,
                VerticalAlignment::Bottom => y = parent_y + parent_height - height,
            }
        }

        // overwrite X if HorizontalAlignment is set
        if let Some(horizontal_alignment) = props.horizontal_alignment {
            match horizontal_alignment {
                HorizontalAlignment::Left => x = parent_x,
                HorizontalAlignment::Center => x = parent_x + (parent_width - width) / 2.0,
                HorizontalAlignment::Right => x = parent_x + parent_width - width,
            }
        }
    }

    let mut paint = Paint::default();
    paint.set_color(props.background_color);

    let rect = SkiaRect::from_xywh(x, y, width, height);
    if let Some(border_radius) = props.border_radius {
        canvas.draw_round_rect(rect, border_radius, border_radius, &paint);
    } else {
        canvas.draw_rect(rect, &paint);
    }
}
