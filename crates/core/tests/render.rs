use std::{fs, path::PathBuf};

use skia_engine_core::{ElementNode, nodes, renderer::render_tree};
use skia_safe::{Color, EncodedImageFormat, Surface, surfaces};

fn surface() -> Surface {
    let mut surface = surfaces::raster_n32_premul((640, 480)).unwrap();
    surface.canvas().clear(Color::BLACK);
    surface
}

fn assert_pixel(surface: &mut Surface, x: i32, y: i32, expected: Color) {
    let pixels = surface.peek_pixels().expect("surface should expose pixels");
    assert_eq!(pixels.get_color((x, y)), expected);
}

fn save_png(surface: &mut Surface, filename: &str) {
    let output_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/core-rendered-tests");
    fs::create_dir_all(&output_dir).expect("create PNG output directory");

    let data = surface
        .image_snapshot()
        .encode(None, EncodedImageFormat::PNG, 100)
        .expect("encode rendered surface as PNG");
    let path = output_dir.join(filename);
    fs::write(&path, data.as_bytes()).expect("write rendered PNG");
    eprintln!("Rendered test image: {}", path.display());
}

#[test]
fn constructs_each_element_node_variant() {
    let container = ElementNode::Container {
        props: nodes::container::Props::default(),
        children: vec![],
    };
    let image = ElementNode::Image {
        props: nodes::image::Props::default(),
    };
    let rect = ElementNode::Rect {
        props: nodes::rect::Props::default(),
    };
    let text = ElementNode::Text {
        props: nodes::text::Props::default(),
        content: "Hello",
    };

    assert!(matches!(container, ElementNode::Container { .. }));
    assert!(matches!(image, ElementNode::Image { .. }));
    assert!(matches!(rect, ElementNode::Rect { .. }));
    assert!(matches!(
        text,
        ElementNode::Text {
            content: "Hello",
            ..
        }
    ));
}

#[test]
fn renders_container_background_and_children() {
    let tree = ElementNode::Container {
        props: nodes::container::Props {
            width: Some(400.0),
            height: Some(300.0),
            x: Some(20.0),
            y: Some(20.0),
            vertical_alignment: None,
            horizontal_alignment: None,
            background_color: Some(Color::BLUE),
            ..Default::default()
        },
        children: vec![ElementNode::Rect {
            props: nodes::rect::Props {
                width: 100.0,
                height: 70.0,
                position: Some(nodes::Position::Relative),
                x: Some(80.0),
                y: Some(60.0),
                vertical_alignment: None,
                horizontal_alignment: None,
                background_color: Color::RED,
                ..Default::default()
            },
        }],
    };
    let mut surface = surface();

    render_tree(surface.canvas(), None, &tree);
    save_png(&mut surface, "container.png");

    assert_pixel(&mut surface, 30, 30, Color::BLUE);
    assert_pixel(&mut surface, 120, 100, Color::RED);
}

#[test]
fn renders_rectangle_fill() {
    let node = ElementNode::Rect {
        props: nodes::rect::Props {
            width: 280.0,
            height: 180.0,
            x: Some(60.0),
            y: Some(50.0),
            vertical_alignment: None,
            horizontal_alignment: None,
            background_color: Color::GREEN,
            ..Default::default()
        },
    };
    let mut surface = surface();

    render_tree(surface.canvas(), None, &node);
    save_png(&mut surface, "rect.png");

    assert_pixel(&mut surface, 200, 150, Color::GREEN);
    assert_pixel(&mut surface, 30, 30, Color::BLACK);
}

#[test]
fn renders_image_background_without_a_source() {
    let node = ElementNode::Image {
        props: nodes::image::Props {
            width: 240.0,
            height: 180.0,
            x: Some(100.0),
            y: Some(100.0),
            vertical_alignment: None,
            horizontal_alignment: None,
            image_src: None,
            background_color: Some(Color::YELLOW),
            ..Default::default()
        },
    };
    let mut surface = surface();

    render_tree(surface.canvas(), None, &node);
    save_png(&mut surface, "image.png");

    assert_pixel(&mut surface, 220, 200, Color::YELLOW);
    assert_pixel(&mut surface, 30, 30, Color::BLACK);
}

#[test]
fn renders_text_glyphs() {
    let node = ElementNode::Text {
        props: nodes::text::Props {
            color: Some(Color::WHITE),
            font_size: Some(72.0),
            x: Some(40.0),
            y: Some(80.0),
            vertical_alignment: None,
            horizontal_alignment: None,
            text_alignment: Some(nodes::TextAlignment::Top),
            ..Default::default()
        },
        content: "Skia",
    };
    let mut surface = surface();

    render_tree(surface.canvas(), None, &node);
    save_png(&mut surface, "text.png");

    let pixels = surface.peek_pixels().expect("surface should expose pixels");
    assert!(
        (0..480).any(|y| (0..640).any(|x| pixels.get_color((x, y)) != Color::BLACK)),
        "rendering text should change at least one pixel"
    );
}
