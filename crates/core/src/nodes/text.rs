use crate::{
    ElementNode,
    nodes::{HorizontalAlignment, Position, TextAlignment, TextOverflow, VerticalAlignment},
};
use skia_safe::{
    Canvas, Color, Font, FontMgr, Rect,
    textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider,
    },
};

const DEFAULT_FONT: &[u8] = include_bytes!("../../fonts/Open Sans/static/OpenSans-Regular.ttf");
const EMBEDDED_FONT_FAMILY: &str = "SkiaEngineEmbeddedFont";

/// Properties that control text appearance, layout, placement, and overflow.
pub struct Props {
    /// Text color; defaults to white.
    pub color: Option<Color>,
    /// Encoded font data. Defaults to the Open Sans font bundled with this
    /// crate. Invalid font data causes rendering to panic.
    pub font_bytes: Vec<u8>,
    /// Font size in pixels; defaults to `16.0`.
    pub font_size: Option<f32>,

    /// Maximum paragraph width in pixels.
    ///
    /// Used as the text box width and as the width constraint when wrapping or
    /// applying single-line ellipsis.
    pub max_width: Option<f32>,
    /// Maximum paragraph height in pixels. Used as the text box height and to
    /// constrain the number of lines in ellipsis mode.
    pub max_height: Option<f32>,

    /// Whether text wraps at the layout width. If enabled without `max_width`,
    /// a positive parent-container width is used when available.
    pub text_wrap: Option<bool>,
    /// How content exceeding the text box is handled.
    pub text_overflow: Option<TextOverflow>,

    /// Positioning mode relative to the parent container.
    pub position: Option<Position>,
    /// Horizontal position in pixels; defaults to `0.0`.
    pub x: Option<f32>,
    /// Vertical position in pixels; defaults to `0.0`.
    pub y: Option<f32>,

    /// Optional vertical placement within the parent container.
    pub vertical_alignment: Option<VerticalAlignment>,
    /// Optional horizontal placement within the parent container.
    pub horizontal_alignment: Option<HorizontalAlignment>,
    /// Vertical placement of the paragraph within its text box.
    pub text_alignment: Option<TextAlignment>,
}

impl Default for Props {
    /// Uses the bundled Open Sans font at 16 pixels, white text, visible
    /// overflow, baseline text alignment, and centered placement defaults for
    /// use as a child.
    fn default() -> Self {
        Self {
            color: Some(Color::WHITE),
            font_bytes: DEFAULT_FONT.to_vec(),
            font_size: Some(16.0),
            max_width: None,
            max_height: None,
            text_wrap: Some(false),
            text_overflow: Some(TextOverflow::Visible),
            position: Some(Position::Absolute),
            x: Some(0.0),
            y: Some(0.0),
            vertical_alignment: Some(VerticalAlignment::Center),
            horizontal_alignment: Some(HorizontalAlignment::Center),
            text_alignment: Some(TextAlignment::Baseline),
        }
    }
}

/// Shapes and paints text on the canvas.
///
/// The paragraph width is constrained by `max_width`, or by the parent
/// container's positive width when wrapping is enabled and no maximum width is
/// supplied. Otherwise, the paragraph is laid out without a finite width.
/// `TextOverflow::Clip` clips to the configured text box; `Ellipsis` requests
/// an ellipsis and clips when finite bounds are available; `Visible` does not
/// clip the paragraph. Vertical text alignment positions the laid-out
/// paragraph inside the selected box.
///
/// An embedded font is registered for this paragraph before layout. Font
/// loading errors panic.
///
/// # Arguments
///
/// * `canvas` - Skia canvas to draw into.
/// * `parent_node` - Parent element, if this text node is nested in a tree.
/// * `props` - Font, layout, placement, color, and overflow properties.
/// * `content` - Text to shape and draw.
pub fn render(canvas: &Canvas, parent_node: Option<&ElementNode>, props: &Props, content: &str) {
    let font_manager = FontMgr::new();
    let typeface = font_manager
        .new_from_bytes(&props.font_bytes, None)
        .expect("Font couldn't be loaded");
    let font = Font::new(typeface.clone(), props.font_size.unwrap_or(16.0));
    let parent_layout = match parent_node {
        Some(ElementNode::Container { props, .. }) => Some((
            props.x.unwrap_or(0.0),
            props.y.unwrap_or(0.0),
            props.width.unwrap_or(0.0),
            props.height.unwrap_or(0.0),
        )),
        _ => None,
    };

    let text_wrap = props.text_wrap.unwrap_or(false);
    let text_overflow = props.text_overflow.unwrap_or(TextOverflow::Visible);
    let parent_width = parent_layout
        .map(|(_, _, width, _)| width)
        .filter(|width| *width > 0.0);
    let box_width = props
        .max_width
        .or_else(|| text_wrap.then_some(parent_width).flatten());
    let layout_width = match (text_wrap, text_overflow, box_width) {
        (true, _, Some(width)) => width,
        (false, TextOverflow::Ellipsis, Some(width)) => width,
        _ => f32::INFINITY,
    };

    let mut text_style = TextStyle::new();
    text_style
        .set_font_size(props.font_size.unwrap_or(16.0))
        .set_color(props.color.unwrap_or_default())
        .set_font_families(&[EMBEDDED_FONT_FAMILY]);

    let mut paragraph_style = ParagraphStyle::new();
    paragraph_style.set_text_style(&text_style);
    if matches!(text_overflow, TextOverflow::Ellipsis) {
        paragraph_style.set_ellipsis("…");
        if !text_wrap && box_width.is_some() {
            paragraph_style.set_max_lines(1);
        } else if let Some(max_height) = props.max_height {
            let (_, metrics) = font.metrics();
            let line_height = metrics.descent - metrics.ascent + metrics.leading;
            if line_height > 0.0 {
                paragraph_style
                    .set_max_lines(Some((max_height / line_height).floor().max(1.0) as usize));
            }
        }
    }

    let mut typeface_provider = TypefaceFontProvider::new();
    typeface_provider.register_typeface(typeface, Some(EMBEDDED_FONT_FAMILY));
    let font_manager: FontMgr = typeface_provider.into();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(font_manager, Some(EMBEDDED_FONT_FAMILY));

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(content);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(layout_width);

    let box_width = box_width.unwrap_or_else(|| paragraph.max_intrinsic_width());
    let box_height = props.max_height.unwrap_or_else(|| paragraph.height());
    let position = props.position.unwrap_or(Position::Absolute);
    let (x, box_top) = layout_origin(
        parent_layout,
        position,
        props.horizontal_alignment,
        props.vertical_alignment,
        (props.x.unwrap_or(0.0), props.y.unwrap_or(0.0)),
        box_width,
        box_height,
    );
    let text_y = box_top
        + text_vertical_offset(
            props.text_alignment.unwrap_or(TextAlignment::Baseline),
            box_height,
            paragraph.height(),
            paragraph.alphabetic_baseline(),
        );

    if matches!(text_overflow, TextOverflow::Clip | TextOverflow::Ellipsis)
        && (box_width.is_finite() || props.max_height.is_some())
    {
        let clip_width = if box_width.is_finite() {
            box_width
        } else {
            f32::MAX
        };
        let clip_height = props.max_height.unwrap_or(f32::MAX);
        canvas.save();
        canvas.clip_rect(
            Rect::from_xywh(x, box_top, clip_width, clip_height),
            None,
            None,
        );
        paragraph.paint(canvas, (x, text_y));
        canvas.restore();
    } else {
        paragraph.paint(canvas, (x, text_y));
    }
}

fn layout_origin(
    parent: Option<(f32, f32, f32, f32)>,
    position: Position,
    horizontal_alignment: Option<HorizontalAlignment>,
    vertical_alignment: Option<VerticalAlignment>,
    coords: (f32, f32),
    width: f32,
    height: f32,
) -> (f32, f32) {
    let (x, y) = coords;
    let Some((parent_x, parent_y, parent_width, parent_height)) = parent else {
        return (x, y);
    };

    let relative_x = match position {
        Position::Absolute => x,
        Position::Relative => parent_x + x,
    };
    let relative_y = match position {
        Position::Absolute => y,
        Position::Relative => parent_y + y,
    };

    let aligned_x = match horizontal_alignment {
        Some(HorizontalAlignment::Left) => Some(parent_x + x),
        Some(HorizontalAlignment::Center) => Some(parent_x + x + (parent_width - width) / 2.0),
        Some(HorizontalAlignment::Right) => Some(parent_x + x + parent_width - width),
        None => None,
    };
    let aligned_y = match vertical_alignment {
        Some(VerticalAlignment::Top) => Some(parent_y + y),
        Some(VerticalAlignment::Center) => Some(parent_y + y + (parent_height - height) / 2.0),
        Some(VerticalAlignment::Bottom) => Some(parent_y + y + parent_height - height),
        None => None,
    };

    (
        aligned_x.unwrap_or(relative_x),
        aligned_y.unwrap_or(relative_y),
    )
}

fn text_vertical_offset(
    alignment: TextAlignment,
    box_height: f32,
    paragraph_height: f32,
    alphabetic_baseline: f32,
) -> f32 {
    match alignment {
        TextAlignment::Baseline => -alphabetic_baseline,
        TextAlignment::Top => 0.0,
        TextAlignment::Middle => (box_height - paragraph_height) / 2.0,
        TextAlignment::Bottom => box_height - paragraph_height,
    }
}
