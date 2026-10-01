use skia_engine_core::{ElementNode, nodes, renderer::render_tree};
use skia_safe::{EncodedImageFormat, surfaces};

#[test]
fn test_render_to_image() {
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
