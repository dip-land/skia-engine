pub mod container;
pub mod rect;

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
