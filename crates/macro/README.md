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

Use RSX-like tags. Attribute values may be Rust expressions, commonly enclosed
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

The supported elements are `Container`, `Rect`, `Image`, and `Text`. `Container`
and `Rect` accept these properties:

| Property               | Container                    | Rect                         |
| ---------------------- | ---------------------------- | ---------------------------- |
| `width`, `height`      | Optional `f32` values        | `f32` values                 |
| `position`             | `nodes::Position`            | `nodes::Position`            |
| `x`, `y`               | Optional `f32` values        | Optional `f32` values        |
| `vertical_alignment`   | `nodes::VerticalAlignment`   | `nodes::VerticalAlignment`   |
| `horizontal_alignment` | `nodes::HorizontalAlignment` | `nodes::HorizontalAlignment` |
| `background_color`     | Optional `skia_safe::Color`  | `skia_safe::Color`           |
| `border_radius`        | Optional `f32` value         | Optional `f32` value         |

`color` is also accepted as an alias for `background_color`. Containers may have
nested elements; `Rect` cannot have children. Omitted properties use each
element's `Default` values (rectangle dimensions default to `0.0` in the macro).
Unknown tags and properties produce compile errors.

`Image` is a leaf element. It accepts `width`, `height`, `position`, `x`, `y`,
`vertical_alignment`, `horizontal_alignment`, `image_src`, `image_sampling`,
`background_color`, and `border_radius`. `image_src` is a string or string
expression; `image_sampling` accepts a `skia_safe::SamplingOptions` value. For
example:

```rust
use skia_engine_macro::skia_ui;

let picture = skia_ui! {
    Image {
        image_src: "assets/photo.png",
        width: 320.0,
        height: 180.0,
        border_radius: 8.0,
    }
};
```

`Text` elements are also supported. They require a `content` string literal and
accept `color` (or `background_color`), `font_bytes` (`Vec<u8>`), `font_size`,
`max_width`, `max_height`, `text_wrap`, `text_overflow`,
`vertical_alignment`, `horizontal_alignment`, `text_alignment`, `position`,
`x`, and `y`. For example:

```rust
use skia_engine_macro::skia_ui;
use skia_safe::Color;

let label = skia_ui! {
    Text {
        content: "A long label that can wrap",
        color: Color::BLACK,
        font_size: 18.0,
        max_width: 160.0,
        text_wrap: true,
        text_overflow: skia_engine_core::nodes::TextOverflow::Ellipsis,
    }
};
```

`Text` is a leaf element; put its contents in `content` rather than nesting
child elements inside it. With `skia_rsx!`, string literal body text can be used
instead of a `content` attribute:

```rust
let label = skia_rsx! {
    <Text font_size={18.0}> "Hello, world!" </Text>
};
```

RSX text must be quoted, as required by `syn-rsx`; unquoted HTML-style text is
not supported. Specify content either in the body or with the `content`
attribute, not both. Font bytes default to the font bundled with
`skia-engine-core`; provide `font_bytes` to use another font.

## Dependency note

The generated code refers directly to `skia_engine_core` and `skia_safe`.
Projects invoking these macros should include both crates as direct
dependencies, in addition to this crate (or the `skia-engine` facade crate).
