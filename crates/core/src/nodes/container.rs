use crate::{
    ElementNode,
    nodes::{HorizontalAlignment, Position, VerticalAlignment},
    renderer::render_tree,
};
use skia_safe::{Canvas, Color, Paint, Rect as SkiaRect};

/// Properties that control a container's size, placement, appearance, and
/// alignment of its children.
///
/// A container can draw an optional background and always renders its child
/// elements. Child positions and alignment are resolved relative to this
/// container when it is their parent.
pub struct Props {
    /// Container width in pixels. When absent, defaults to `10.0`.
    pub width: Option<f32>,
    /// Container height in pixels. When absent, defaults to `10.0`.
    pub height: Option<f32>,

    /// Whether this container's position is relative to its parent or absolute.
    ///
    /// For child containers, `Relative` adds the parent's origin to `x` and
    /// `y`. `Absolute` uses those coordinates directly. Defaults to
    /// [`Position::Absolute`].
    pub position: Option<Position>,
    /// Horizontal position in pixels. When absent, defaults to `0.0`.
    pub x: Option<f32>,
    /// Vertical position in pixels. When absent, defaults to `0.0`.
    pub y: Option<f32>,

    /// Vertical placement of this container within its parent, when present.
    ///
    /// For child containers this takes precedence over `y` and `position`.
    pub vertical_alignment: Option<VerticalAlignment>,
    /// Horizontal placement of this container within its parent, when present.
    ///
    /// For child containers this takes precedence over `x` and `position`.
    pub horizontal_alignment: Option<HorizontalAlignment>,

    /// Optional fill color. If `None`, no background is drawn.
    pub background_color: Option<Color>,
    /// Optional corner radius in pixels for the background.
    pub border_radius: Option<f32>,
}

impl Default for Props {
    /// Creates a 10-by-10 container at the origin, positioned absolutely and
    /// centered within its parent when rendered as a child. The background and
    /// border radius are unset.
    fn default() -> Self {
        Self {
            width: Some(10.0),
            height: Some(10.0),
            position: Some(Position::Absolute),
            x: Some(0.0),
            y: Some(0.0),
            vertical_alignment: Some(VerticalAlignment::Center),
            horizontal_alignment: Some(HorizontalAlignment::Center),
            background_color: None,
            border_radius: None,
        }
    }
}

/// Draws a container's background, if configured, and recursively renders its
/// children.
///
/// A child container's relative coordinates are offset by the parent's
/// coordinates. If either alignment is set, that axis is instead aligned to
/// the parent and its explicit coordinate is ignored. For a root container,
/// coordinates are used without parent-relative positioning or alignment.
///
/// The background is drawn before the children, so children appear on top.
///
/// # Arguments
///
/// * `canvas` - Skia canvas to draw into.
/// * `parent_node` - Parent element, if this container is nested in a tree.
/// * `node` - The container element, passed to child rendering as their parent.
/// * `props` - Size, placement, and appearance properties for this container.
/// * `children` - Elements to render inside this container.
pub fn render(
    canvas: &Canvas,
    parent_node: Option<&ElementNode>,
    node: &ElementNode,
    props: &Props,
    children: &Vec<ElementNode>,
) {
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
    let width = props.width.unwrap_or(10.0);
    let height = props.height.unwrap_or(10.0);

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

    // Only draw if background_color is set
    if let Some(bg_color) = props.background_color {
        let mut paint = Paint::default();
        paint.set_color(bg_color);

        let rect = SkiaRect::from_xywh(x, y, width, height);
        if let Some(border_radius) = props.border_radius {
            canvas.draw_round_rect(rect, border_radius, border_radius, &paint);
        } else {
            canvas.draw_rect(rect, &paint);
        }
    }

    for child in children {
        render_tree(canvas, Some(node), child);
    }
}
