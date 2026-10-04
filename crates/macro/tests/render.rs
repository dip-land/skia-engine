use std::{fs, path::PathBuf};

use skia_engine_core::{ElementNode, renderer::render_tree};
use skia_engine_macro::{skia_rsx, skia_ui};
use skia_safe::{Color, EncodedImageFormat, Surface, surfaces};

fn surface() -> Surface {
    let mut surface = surfaces::raster_n32_premul((640, 480)).expect("create raster surface");
    surface.canvas().clear(Color::BLACK);
    surface
}

fn render_and_save(node: &ElementNode, filename: &str) {
    let mut surface = surface();
    render_tree(surface.canvas(), None, node);

    let pixels = surface.peek_pixels().expect("rendered surface has pixels");
    assert_eq!(pixels.get_color((320, 250)), Color::BLUE);
    assert_eq!(pixels.get_color((50, 50)), Color::RED);
    assert_eq!(pixels.get_color((500, 50)), Color::YELLOW);
    assert!(
        (340..460).any(|y| { (20..320).any(|x| pixels.get_color((x, y)) != Color::BLUE) }),
        "rendered Text node should draw glyphs over the container background"
    );
    drop(pixels);

    let output_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/macro-rendered-tests");
    fs::create_dir_all(&output_dir).expect("create PNG output directory");
    let data = surface
        .image_snapshot()
        .encode(None, EncodedImageFormat::PNG, 100)
        .expect("encode rendered surface as PNG");
    let path = output_dir.join(filename);
    fs::write(&path, data.as_bytes()).expect("write rendered PNG");
    eprintln!("Rendered macro test image: {}", path.display());
}

#[test]
fn renders_every_node_from_skia_ui_macro() {
    let tree: ElementNode = skia_ui! {
        Container {
            width: 620.0,
            height: 460.0,
            x: 10.0,
            y: 10.0,
            background_color: Color::BLUE,
            Rect {
                width: 240.0,
                height: 180.0,
                vertical_alignment: skia_engine_core::nodes::VerticalAlignment::Top,
                horizontal_alignment: skia_engine_core::nodes::HorizontalAlignment::Left,
                background_color: Color::RED,
            },
            Image {
                width: 200.0,
                height: 150.0,
                vertical_alignment: skia_engine_core::nodes::VerticalAlignment::Top,
                horizontal_alignment: skia_engine_core::nodes::HorizontalAlignment::Right,
                background_color: Color::YELLOW,
            },
            Text {
                content: "skia_ui!",
                font_size: 56.0,
                vertical_alignment: skia_engine_core::nodes::VerticalAlignment::Bottom,
                horizontal_alignment: skia_engine_core::nodes::HorizontalAlignment::Left,
                text_alignment: skia_engine_core::nodes::TextAlignment::Top,
                color: Color::WHITE,
            },
        }
    };

    render_and_save(&tree, "skia-ui.png");
}

#[test]
fn renders_every_node_from_skia_rsx_macro() {
    let tree: ElementNode = skia_rsx! {
        <Container
            width={620.0}
            height={460.0}
            x={10.0}
            y={10.0}
            background_color=Color::BLUE
        >
            <Rect
                width={240.0}
                height={180.0}
                vertical_alignment=skia_engine_core::nodes::VerticalAlignment::Top
                horizontal_alignment=skia_engine_core::nodes::HorizontalAlignment::Left
                background_color={Color::RED}
            />
            <Image
                width={200.0}
                height={150.0}
                vertical_alignment=skia_engine_core::nodes::VerticalAlignment::Top
                horizontal_alignment=skia_engine_core::nodes::HorizontalAlignment::Right
                background_color=Color::YELLOW
            />
            <Text
                font_size={56.0}
                vertical_alignment=skia_engine_core::nodes::VerticalAlignment::Bottom
                horizontal_alignment=skia_engine_core::nodes::HorizontalAlignment::Left
                text_alignment=skia_engine_core::nodes::TextAlignment::Top
                color=Color::WHITE
            >
                "skia_rsx!"
            </Text>
        </Container>
    };

    render_and_save(&tree, "skia-rsx.png");
}
