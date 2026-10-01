use skia_engine_core::{ElementNode, renderer::render_tree};
use skia_engine_macro::{skia_rsx, skia_ui};
use skia_safe::{Color, EncodedImageFormat, surfaces};

#[test]
fn test_skia_ui_render() {
    let node: ElementNode = skia_ui! {
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
    let canvas = surface.canvas();

    render_tree(canvas, None, &node);

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let data = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();

    std::fs::write("skia_ui_render.png", data.as_bytes()).unwrap();
}

#[test]
fn test_skia_rsx_render() {
    let node: ElementNode = skia_rsx! {
        <Container width = {400.0} height = {300.0} background_color = Color::WHITE>
            <Rect width = {100.0} height = {100.0} background_color = Color::BLUE />
        </Container>
    };

    let mut surface = surfaces::raster_n32_premul((400, 300)).unwrap();
    let canvas = surface.canvas();

    render_tree(canvas, None, &node);

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let data = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();

    std::fs::write("skia_rsx_render.png", data.as_bytes()).unwrap();
}
