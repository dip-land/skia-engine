use skia_engine_core::{
    ElementNode, nodes,
    nodes::{HorizontalAlignment, TextAlignment, TextOverflow, VerticalAlignment},
    renderer::render_tree,
};
use skia_safe::{EncodedImageFormat, surfaces};

#[test]
fn render_to_image() {
    let root_node = ElementNode::Container {
        props: nodes::container::Props {
            width: Some(400.0),
            height: Some(300.0),
            background_color: Some(skia_safe::Color::WHITE),
            ..Default::default()
        },
        children: vec![ElementNode::Rect {
            props: nodes::rect::Props {
                width: 100.0,
                height: 100.0,
                background_color: skia_safe::Color::BLUE,
                ..Default::default()
            },
        }],
    };

    let mut surface = surfaces::raster_n32_premul((400, 300)).unwrap();
    let canvas = surface.canvas();

    render_tree(canvas, None, &root_node);

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let data = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();

    std::fs::write("render.png", data.as_bytes()).unwrap();
}

#[test]
fn text_wrapping_and_overflow_modes() {
    let parent = ElementNode::Container {
        props: nodes::container::Props {
            width: Some(120.0),
            height: Some(60.0),
            ..Default::default()
        },
        children: vec![],
    };
    let cases = [
        (true, TextOverflow::Visible),
        (true, TextOverflow::Clip),
        (true, TextOverflow::Ellipsis),
        (false, TextOverflow::Ellipsis),
    ];
    let mut surface = surfaces::raster_n32_premul((160, 100)).unwrap();
    let canvas = surface.canvas();

    for (text_wrap, text_overflow) in cases {
        let text = ElementNode::Text {
            props: nodes::text::Props {
                color: Some(skia_safe::Color::BLACK),
                max_width: Some(60.0),
                max_height: Some(24.0),
                text_wrap: Some(text_wrap),
                text_overflow: Some(text_overflow),
                horizontal_alignment: Some(HorizontalAlignment::Left),
                vertical_alignment: Some(VerticalAlignment::Top),
                text_alignment: Some(TextAlignment::Top),
                ..Default::default()
            },
            content: "A long line of text that should wrap or overflow the text box.",
        };

        render_tree(canvas, Some(&parent), &text);
    }
}

#[test]
fn rendering_images() {
    let root_node = ElementNode::Container {
        props: nodes::container::Props {
            width: Some(400.0),
            height: Some(300.0),
            background_color: Some(skia_safe::Color::WHITE),
            ..Default::default()
        },
        children: vec![ElementNode::Image {
            props: nodes::image::Props {
                width: 100.0,
                height: 100.0,
                image_src: Some(
                    "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQbKXzqnplp0IkqASMkqHGHQg1A0-OiFvk89YFRDeNHYg&s=10"
                        .to_string(),
                ),
                background_color: Some(skia_safe::Color::BLUE),
                ..Default::default()
            },
        }],
    };

    let mut surface = surfaces::raster_n32_premul((400, 300)).unwrap();
    let canvas = surface.canvas();

    render_tree(canvas, None, &root_node);

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let data = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();

    std::fs::write("rendering_images.png", data.as_bytes()).unwrap();
}
