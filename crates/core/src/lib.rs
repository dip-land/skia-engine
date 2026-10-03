pub mod nodes;
pub mod renderer;

pub enum ElementNode {
    Container {
        props: nodes::container::Props,
        children: Vec<ElementNode>,
    },
    Rect {
        props: nodes::rect::Props,
    },
    Text {
        props: nodes::text::Props,
        content: &'static str,
    },
}
