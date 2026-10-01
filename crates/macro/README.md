# skia-engine-macro

This crate provides the `skia_ui!` and `skia_rsx!` procedural macros. Both
macros generate `skia_engine_core::ElementNode` values for the core renderer.

## `skia_ui!`

Use Rust-like brace syntax, with a comma after each property and nested element:

```rust
use skia_engine_macro::skia_ui;
use skia_safe::Color;

let tree = skia_ui! {
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
```

## `skia_rsx!`

Use JSX-like tags. Attribute values may be Rust expressions, commonly enclosed
in braces:

```rust
use skia_engine_macro::skia_rsx;
use skia_safe::Color;

let tree = skia_rsx! {
    <Container width={400.0} height={300.0} background_color={Color::WHITE}>
        <Rect width={100.0} height={100.0} background_color={Color::BLUE} />
    </Container>
};
```

## Supported elements and properties

The supported elements are `Container` and `Rect`. Both macros accept these
properties:

| Property | Container | Rect |
|---|---|---|
| `width`, `height` | Optional `f32` values | `f32` values |
| `position` | `nodes::Position` | `nodes::Position` |
| `x`, `y` | Optional `f32` values | Optional `f32` values |
| `vertical_alignment` | `nodes::VerticalAlignment` | `nodes::VerticalAlignment` |
| `horizontal_alignment` | `nodes::HorizontalAlignment` | `nodes::HorizontalAlignment` |
| `background_color` | Optional `skia_safe::Color` | `skia_safe::Color` |
| `border_radius` | Optional `f32` value | Optional `f32` value |

`color` is also accepted as an alias for `background_color`. Containers may have
nested elements; `Rect` cannot have children. Omitted properties use each
element's `Default` values (rectangle dimensions default to `0.0` in the macro).
Unknown tags and properties produce compile errors.

## Dependency note

The generated code refers directly to `skia_engine_core` and `skia_safe`.
Projects invoking these macros should include both crates as direct
dependencies, in addition to this crate (or the `skia-engine` facade crate).
