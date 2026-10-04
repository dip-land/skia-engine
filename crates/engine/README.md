# skia-engine

The `skia-engine` crate is the public-facing crate. It re-exports the core
element types, node properties, renderer, and UI macros.

## Usage

```rust
use skia_engine::{ElementNode, skia_ui};
use skia_engine::nodes;
use skia_safe::{Color, surfaces};

let tree: ElementNode = skia_ui! {
    Container {
        width: 400.0,
        height: 300.0,
        background_color: Color::WHITE,
        Rect {
            width: 100.0,
            height: 100.0,
            background_color: Color::BLUE,
        }
    }
};

let mut surface = surfaces::raster_n32_premul((400, 300)).unwrap();
skia_engine::render_tree(surface.canvas(), None, &tree);
```

To use the example, add `skia-engine` and `skia-safe` to your application's
dependencies. The macros currently expand to paths in `skia_engine_core`, so
applications using the macros also need `skia-engine-core` as a direct
dependency.

## RSX-like syntax with `skia_rsx!`

Use `skia_rsx!` when you prefer tag-based syntax. Put Rust expressions in
attribute values (braces are supported), and nest child elements inside their
parent:

```rust
use skia_engine::{ElementNode, skia_rsx};
use skia_safe::Color;

let tree: ElementNode = skia_rsx! {
    <Container width={400.0} height={300.0} background_color=Color::WHITE>
        <Rect width={100.0} height={100.0} background_color=Color::BLUE />
    </Container>
};
```

`skia_rsx!` currently supports `Container` and `Rect` elements and the same
properties as `skia_ui!`. See the `skia-engine-macro` README for the complete
property list and defaults.

## Re-exports

- `ElementNode` and `nodes` for constructing or inspecting element trees.
- `render_tree` for drawing an element tree to a Skia canvas.
- `skia_ui!` for brace-based nested element syntax.
- `skia_rsx!` for RSX-like element syntax.

See the `skia-engine-macro` crate README for supported elements and properties.
