use crate::{
    ElementNode,
    nodes::{HorizontalAlignment, Position, VerticalAlignment},
};
use skia_safe::{Canvas, Color, Data, Image, Paint, RRect, Rect as SkiaRect, SamplingOptions};

/// Properties that control an image node's size, placement, source, and
/// appearance.
pub struct Props {
    /// Display width in pixels.
    pub width: f32,
    /// Display height in pixels.
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

    /// Path to an encoded image file or an HTTP(S) URL.
    ///
    /// URL loading requires exactly one of the `reqwest` or `ureq` crate
    /// features. Local file paths do not require an HTTP feature.
    pub image_src: Option<String>,
    /// Skia sampling configuration used when scaling the image.
    pub image_sampling: Option<SamplingOptions>,
    /// Optional color applied to the rounded backing shape drawn under the
    /// image.
    pub background_color: Option<Color>,
    /// Optional corner radius in pixels. The image is clipped to this rounded
    /// shape when it is set.
    pub border_radius: Option<f32>,
}

impl Default for Props {
    /// Creates a 10-by-10 image node at the origin with default sampling,
    /// centered alignment defaults for use as a child, and no source.
    fn default() -> Self {
        Self {
            width: 10.0,
            height: 10.0,
            position: Some(Position::Absolute),
            x: Some(0.0),
            y: Some(0.0),
            vertical_alignment: Some(VerticalAlignment::Center),
            horizontal_alignment: Some(HorizontalAlignment::Center),
            image_src: None,
            image_sampling: Some(skia_safe::SamplingOptions::default()),
            background_color: None,
            border_radius: None,
        }
    }
}

/// Loads and draws an image node.
///
/// The source may be a local file path or an HTTP(S) URL. Encoded image bytes
/// are decoded by Skia, scaled to the node's bounds using `image_sampling`,
/// and clipped to a rounded rectangle when `border_radius` is set. If no
/// source is provided, the backing shape is still drawn.
///
/// HTTP loading is enabled by the crate's `reqwest` or `ureq` feature; enabling
/// both is a compile-time error. The `enable-image-errors` feature makes
/// loading and decoding failures explicit, which is useful when diagnosing
/// invalid paths, failed requests, or unsupported image data.
///
/// For child nodes, relative positioning adds the parent container's origin.
/// Configured horizontal or vertical alignment takes precedence over the
/// corresponding coordinate and positioning mode.
///
/// # Arguments
///
/// * `canvas` - Skia canvas to draw into.
/// * `parent_node` - Parent element, if this image is nested in a tree.
/// * `props` - Size, placement, source, sampling, and appearance properties.
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

    let border_radius = props.border_radius.unwrap_or(0.0);

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
    if let Some(background_color) = props.background_color {
        paint.set_color(background_color);
    }

    let rect = SkiaRect::from_xywh(x, y, width, height);
    let rect = RRect::new_rect_xy(rect, border_radius, border_radius);

    let image = if let Some(image_src) = props.image_src.as_deref() {
        if image_src.starts_with("http://") || image_src.starts_with("https://") {
            #[cfg(all(feature = "reqwest", feature = "ureq"))]
            {
                panic!("Only 1 http client feature can be selected!");
            }

            #[cfg(all(
                not(feature = "reqwest"),
                not(feature = "ureq"),
                feature = "enable-image-errors"
            ))]
            {
                panic!("At least one http client feature must be selected to use image URLs!");
            }

            #[cfg(all(
                not(feature = "reqwest"),
                not(feature = "ureq"),
                not(feature = "enable-image-errors")
            ))]
            {
                None
            }

            #[cfg(all(feature = "reqwest", feature = "enable-image-errors"))]
            {
                let response = reqwest::blocking::get(image_src)
                    .expect("Failed to fetch image")
                    .error_for_status()
                    .expect("Image URL returned an unsuccessful HTTP status");

                let bytes = response.bytes().expect("Failed to read image response");

                let image = Image::from_encoded(Data::new_copy(bytes.as_ref()))
                    .expect("Response body is not a supported image format");
                Some(image)
            }

            #[cfg(all(feature = "reqwest", not(feature = "enable-image-errors")))]
            {
                match reqwest::blocking::get(image_src) {
                    Ok(res) => {
                        let bytes = res.bytes().unwrap();
                        Image::from_encoded(Data::new_copy(&bytes))
                    }
                    Err(_) => None,
                }
            }

            #[cfg(all(feature = "ureq", feature = "enable-image-errors"))]
            {
                let res = ureq::get(image_src).call().expect("Failed to fetch image");
                let bytes = res.into_body().read_to_vec().unwrap();
                let image = Image::from_encoded(Data::new_copy(&bytes))
                    .expect("Skia could not decode the image");
                Some(image)
            }

            #[cfg(all(feature = "ureq", not(feature = "enable-image-errors")))]
            {
                let res = ureq::get(image_src)
                    .call()
                    .unwrap_or(ureq::http::Response::new(ureq::Body::builder().data("")));
                let bytes = res.into_body().read_to_vec().unwrap_or(Vec::new());
                Image::from_encoded(Data::new_copy(&bytes))
            }
        } else {
            #[cfg(not(feature = "enable-image-errors"))]
            {
                let bytes = std::fs::read(image_src).unwrap_or(Vec::new());
                Image::from_encoded(Data::new_copy(&bytes))
            }

            #[cfg(feature = "enable-image-errors")]
            {
                let bytes = std::fs::read(image_src).expect("Could not read image file");
                let image = Image::from_encoded(Data::new_copy(&bytes))
                    .expect("Skia could not decode the image");
                Some(image)
            }
        }
    } else {
        None
    };

    let sampling = props.image_sampling.unwrap_or_default();

    canvas.draw_rrect(rect, &paint);
    if let Some(image) = image {
        canvas.save();
        canvas.clip_rrect(rect, None, None);
        canvas.draw_image_rect_with_sampling_options(image, None, rect.rect(), sampling, &paint);
        canvas.restore();
    }
}
