# skia-engine-core

The core crate provides the element tree and Skia renderer used by `skia-engine`.
Use it directly when you want to construct UI elements without the procedural
macros.

## Elements

`ElementNode` currently supports:

- `Container`, with a `Vec<ElementNode>` of children.
- `Rect`, which represents a filled rectangle.

Both node types have a `Props` type with dimensions, position, alignment, color,
and border-radius settings. Container dimensions and background color are
optional. Rectangle dimensions and color have concrete values.

## Example

```rust
use skia_engine_core::{
    ElementNode,
    nodes,
    renderer::render_tree,
};
use skia_safe::{Color, surfaces};

let tree = ElementNode::Container {
    props: nodes::container::Props {
        width: Some(400.0),
        height: Some(300.0),
        background_color: Some(Color::WHITE),
        ..Default::default()
    },
    children: vec![ElementNode::Rect {
        props: nodes::rect::Props {
            width: 100.0,
            height: 100.0,
            background_color: Color::BLUE,
            ..Default::default()
        },
    }],
};

let mut surface = surfaces::raster_n32_premul((400, 300)).unwrap();
render_tree(surface.canvas(), None, &tree);
```

Call `render_tree` with the Skia canvas, an optional parent element, and the root
element to draw the tree.
