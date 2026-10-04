pub mod container;
pub mod image;
pub mod rect;
pub mod text;

#[derive(Debug, Clone, Copy)]
pub enum Position {
    Absolute,
    Relative,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum TextAlignment {
    Baseline,
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub enum TextOverflow {
    Visible,
    Clip,
    Ellipsis,
}

pub use skia_safe::SamplingOptions;
