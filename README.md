# skia-engine

A Rust UI and 2D drawing library built on [Skia](https://skia.org/). It provides
a small element tree, a renderer, and procedural macros for declaring nested
elements.

## Crates

| Crate | Purpose |
|---|---|
| [`skia-engine`](crates/engine) | Public facade that re-exports the element tree, renderer, and macros. |
| [`skia-engine-core`](crates/core) | Element types, layout properties, and Skia rendering implementation. |
| [`skia-engine-macro`](crates/macro) | `skia_ui!` and `skia_rsx!` procedural macros. |

## Getting started

The facade crate re-exports the macros and core types. Macro expansions refer
to `skia_engine_core` and `skia_safe`, so include those dependencies in your
application as well:

```toml
[dependencies]
skia-engine = "0.1.1"
skia-engine-core = "0.1.1"
skia-safe = "0.153.3"
```

Declare a nested element tree with `skia_ui!`:

```rust
use skia_engine::{ElementNode, render_tree, skia_ui};
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
render_tree(surface.canvas(), None, &tree);
```

Alternatively, use `skia_rsx!` for JSX-like syntax:

```rust
use skia_engine::{ElementNode, skia_rsx};
use skia_safe::Color;

let tree: ElementNode = skia_rsx! {
    <Container width={400.0} height={300.0} background_color=Color::WHITE>
        <Rect width={100.0} height={100.0} background_color=Color::BLUE />
    </Container>
};
```

## Supported elements and properties

The macros currently support `Container` and `Rect`. Both accept `width`,
`height`, `position`, `x`, `y`, `vertical_alignment`, `horizontal_alignment`,
`background_color`, and `border_radius`. `color` is an alias for
`background_color`. Containers can contain child elements; rectangles cannot.

Position and alignment values are provided by `skia_engine_core::nodes`:

```rust
skia_engine_core::nodes::Position::Relative
skia_engine_core::nodes::HorizontalAlignment::Center
skia_engine_core::nodes::VerticalAlignment::Top
```

See the crate-specific READMEs for detailed property types, default behavior,
and usage examples:

- [`skia-engine`](crates/engine/README.md)
- [`skia-engine-core`](crates/core/README.md)
- [`skia-engine-macro`](crates/macro/README.md)

## Build and test

Run the workspace tests from the repository root:

```sh
cargo test --workspace
```

## License

Licensed under the MIT license.
