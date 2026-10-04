use skia_engine_core::ElementNode;
use skia_engine_macro::{skia_rsx, skia_ui};
use skia_safe::Color;

#[test]
fn test_skia_ui() {
    let node: ElementNode = skia_ui! {
        Container {
            width: 150.0,
            height: 250.0,
            background_color: Color::WHITE,

            Rect {
                width: 120.0,
                height: 80.0,
                background_color: Color::RED,
            }
        }
    };

    match node {
        ElementNode::Container { props, children } => {
            assert_eq!(props.width, Some(150.0));
            assert_eq!(props.height, Some(250.0));
            assert_eq!(props.background_color, Some(Color::WHITE));
            assert_eq!(children.len(), 1);
            match &children[0] {
                ElementNode::Rect { props } => {
                    assert_eq!(props.width, 120.0);
                    assert_eq!(props.height, 80.0);
                    assert_eq!(props.background_color, Color::RED);
                }
                _ => panic!("Expected a nested ElementNode::Rect variant"),
            }
        }
        _ => panic!("Expected an ElementNode::View variant"),
    }
}

#[test]
fn test_skia_rsx() {
    let node: ElementNode = skia_rsx! {
        <Container width = {150.0} height = {250.0} background_color = Color::WHITE>
            <Rect width = {120.0} height = {80.0} background_color = Color::RED />
        </Container>
    };

    match node {
        ElementNode::Container { props, children } => {
            assert_eq!(props.width, Some(150.0));
            assert_eq!(props.height, Some(250.0));
            assert_eq!(props.background_color, Some(Color::WHITE));

            // Check that the nested Rect child was successfully parsed and expanded
            assert_eq!(children.len(), 1);
            match &children[0] {
                ElementNode::Rect { props: rect_props } => {
                    assert_eq!(rect_props.width, 120.0);
                    assert_eq!(rect_props.height, 80.0);
                    assert_eq!(rect_props.background_color, Color::RED);
                }
                _ => panic!("Expected a nested ElementNode::Rect variant"),
            }
        }
        _ => panic!("Expected an ElementNode::Container variant"),
    }
}

#[test]
fn test_skia_ui_all_properties() {
    let node: ElementNode = skia_ui! {
        Container {
            width: 200.0,
            height: 100.0,
            position: skia_engine_core::nodes::Position::Relative,
            x: 12.0,
            y: 18.0,
            vertical_alignment: skia_engine_core::nodes::VerticalAlignment::Bottom,
            horizontal_alignment: skia_engine_core::nodes::HorizontalAlignment::Right,
            background_color: Color::WHITE,
            border_radius: 6.0,
            Rect {
                width: 40.0,
                height: 30.0,
                position: skia_engine_core::nodes::Position::Relative,
                x: 4.0,
                y: 5.0,
                vertical_alignment: skia_engine_core::nodes::VerticalAlignment::Top,
                horizontal_alignment: skia_engine_core::nodes::HorizontalAlignment::Left,
                background_color: Color::RED,
                border_radius: 3.0,
            }
        }
    };

    let ElementNode::Container { props, children } = node else {
        panic!("Expected an ElementNode::Container variant");
    };
    assert_eq!(props.width, Some(200.0));
    assert_eq!(props.height, Some(100.0));
    assert!(matches!(
        props.position,
        Some(skia_engine_core::nodes::Position::Relative)
    ));
    assert_eq!(props.x, Some(12.0));
    assert_eq!(props.y, Some(18.0));
    assert!(matches!(
        props.vertical_alignment,
        Some(skia_engine_core::nodes::VerticalAlignment::Bottom)
    ));
    assert!(matches!(
        props.horizontal_alignment,
        Some(skia_engine_core::nodes::HorizontalAlignment::Right)
    ));
    assert_eq!(props.background_color, Some(Color::WHITE));
    assert_eq!(props.border_radius, Some(6.0));

    let ElementNode::Rect { props } = &children[0] else {
        panic!("Expected a nested ElementNode::Rect variant");
    };
    assert_eq!(props.width, 40.0);
    assert_eq!(props.height, 30.0);
    assert!(matches!(
        props.position,
        Some(skia_engine_core::nodes::Position::Relative)
    ));
    assert_eq!(props.x, Some(4.0));
    assert_eq!(props.y, Some(5.0));
    assert!(matches!(
        props.vertical_alignment,
        Some(skia_engine_core::nodes::VerticalAlignment::Top)
    ));
    assert!(matches!(
        props.horizontal_alignment,
        Some(skia_engine_core::nodes::HorizontalAlignment::Left)
    ));
    assert_eq!(props.background_color, Color::RED);
    assert_eq!(props.border_radius, Some(3.0));
}

#[test]
fn test_skia_rsx_all_properties() {
    let node: ElementNode = skia_rsx! {
        <Container
            width={200.0}
            height={100.0}
            position=skia_engine_core::nodes::Position::Relative
            x={12.0}
            y={18.0}
            vertical_alignment=skia_engine_core::nodes::VerticalAlignment::Bottom
            horizontal_alignment=skia_engine_core::nodes::HorizontalAlignment::Right
            background_color=Color::WHITE
            border_radius={6.0}
        >
            <Rect
                width={40.0}
                height={30.0}
                position=skia_engine_core::nodes::Position::Relative
                x={4.0}
                y={5.0}
                vertical_alignment=skia_engine_core::nodes::VerticalAlignment::Top
                horizontal_alignment=skia_engine_core::nodes::HorizontalAlignment::Left
                background_color=Color::RED
                border_radius={3.0}
            />
        </Container>
    };

    let ElementNode::Container { props, children } = node else {
        panic!("Expected an ElementNode::Container variant");
    };
    assert_eq!(props.width, Some(200.0));
    assert_eq!(props.height, Some(100.0));
    assert!(matches!(
        props.position,
        Some(skia_engine_core::nodes::Position::Relative)
    ));
    assert_eq!(props.x, Some(12.0));
    assert_eq!(props.y, Some(18.0));
    assert!(matches!(
        props.vertical_alignment,
        Some(skia_engine_core::nodes::VerticalAlignment::Bottom)
    ));
    assert!(matches!(
        props.horizontal_alignment,
        Some(skia_engine_core::nodes::HorizontalAlignment::Right)
    ));
    assert_eq!(props.background_color, Some(Color::WHITE));
    assert_eq!(props.border_radius, Some(6.0));

    let ElementNode::Rect { props } = &children[0] else {
        panic!("Expected a nested ElementNode::Rect variant");
    };
    assert_eq!(props.width, 40.0);
    assert_eq!(props.height, 30.0);
    assert!(matches!(
        props.position,
        Some(skia_engine_core::nodes::Position::Relative)
    ));
    assert_eq!(props.x, Some(4.0));
    assert_eq!(props.y, Some(5.0));
    assert!(matches!(
        props.vertical_alignment,
        Some(skia_engine_core::nodes::VerticalAlignment::Top)
    ));
    assert!(matches!(
        props.horizontal_alignment,
        Some(skia_engine_core::nodes::HorizontalAlignment::Left)
    ));
    assert_eq!(props.background_color, Color::RED);
    assert_eq!(props.border_radius, Some(3.0));
}

#[test]
fn test_skia_ui_text_node() {
    let node: ElementNode = skia_ui! {
        Text {
            content: "Wrapped text",
            color: Color::BLACK,
            font_size: 18.0,
            max_width: 120.0,
            max_height: 48.0,
            text_wrap: true,
            text_overflow: skia_engine_core::nodes::TextOverflow::Ellipsis,
            position: skia_engine_core::nodes::Position::Relative,
            x: 8.0,
            y: 12.0,
            vertical_alignment: skia_engine_core::nodes::VerticalAlignment::Top,
            horizontal_alignment: skia_engine_core::nodes::HorizontalAlignment::Left,
            text_alignment: skia_engine_core::nodes::TextAlignment::Middle,
        }
    };

    let ElementNode::Text { props, content } = node else {
        panic!("Expected an ElementNode::Text variant");
    };
    assert_eq!(content, "Wrapped text");
    assert_eq!(props.color, Some(Color::BLACK));
    assert_eq!(props.font_size, Some(18.0));
    assert_eq!(props.max_width, Some(120.0));
    assert_eq!(props.max_height, Some(48.0));
    assert_eq!(props.text_wrap, Some(true));
    assert!(matches!(
        props.text_overflow,
        Some(skia_engine_core::nodes::TextOverflow::Ellipsis)
    ));
    assert!(matches!(
        props.position,
        Some(skia_engine_core::nodes::Position::Relative)
    ));
    assert_eq!(props.x, Some(8.0));
    assert_eq!(props.y, Some(12.0));
    assert!(matches!(
        props.vertical_alignment,
        Some(skia_engine_core::nodes::VerticalAlignment::Top)
    ));
    assert!(matches!(
        props.horizontal_alignment,
        Some(skia_engine_core::nodes::HorizontalAlignment::Left)
    ));
    assert!(matches!(
        props.text_alignment,
        Some(skia_engine_core::nodes::TextAlignment::Middle)
    ));
}

#[test]
fn test_skia_rsx_text_node() {
    let node: ElementNode = skia_rsx! {
        <Text
            color=Color::BLACK
            font_size={18.0}
            max_width={120.0}
            max_height={48.0}
            text_wrap=true
            text_overflow=skia_engine_core::nodes::TextOverflow::Clip
            position=skia_engine_core::nodes::Position::Relative
            x={8.0}
            y={12.0}
            vertical_alignment=skia_engine_core::nodes::VerticalAlignment::Top
            horizontal_alignment=skia_engine_core::nodes::HorizontalAlignment::Left
            text_alignment=skia_engine_core::nodes::TextAlignment::Bottom
        >"Wrapped text"</Text>
    };

    let ElementNode::Text { props, content } = node else {
        panic!("Expected an ElementNode::Text variant");
    };
    assert_eq!(content, "Wrapped text");
    assert_eq!(props.color, Some(Color::BLACK));
    assert_eq!(props.font_size, Some(18.0));
    assert_eq!(props.max_width, Some(120.0));
    assert_eq!(props.max_height, Some(48.0));
    assert_eq!(props.text_wrap, Some(true));
    assert!(matches!(
        props.text_overflow,
        Some(skia_engine_core::nodes::TextOverflow::Clip)
    ));
    assert!(matches!(
        props.position,
        Some(skia_engine_core::nodes::Position::Relative)
    ));
    assert_eq!(props.x, Some(8.0));
    assert_eq!(props.y, Some(12.0));
    assert!(matches!(
        props.vertical_alignment,
        Some(skia_engine_core::nodes::VerticalAlignment::Top)
    ));
    assert!(matches!(
        props.horizontal_alignment,
        Some(skia_engine_core::nodes::HorizontalAlignment::Left)
    ));
    assert!(matches!(
        props.text_alignment,
        Some(skia_engine_core::nodes::TextAlignment::Bottom)
    ));
}

#[test]
fn test_skia_ui_image_node() {
    let sampling = skia_engine_core::nodes::SamplingOptions::new(
        skia_safe::FilterMode::Linear,
        skia_safe::MipmapMode::None,
    );
    let node: ElementNode = skia_ui! {
        Image {
            width: 160.0,
            height: 90.0,
            position: skia_engine_core::nodes::Position::Relative,
            x: 8.0,
            y: 12.0,
            vertical_alignment: skia_engine_core::nodes::VerticalAlignment::Top,
            horizontal_alignment: skia_engine_core::nodes::HorizontalAlignment::Left,
            image_src: "assets/photo.png",
            image_sampling: sampling,
            background_color: Color::WHITE,
            border_radius: 6.0,
        }
    };

    let ElementNode::Image { props } = node else {
        panic!("Expected an ElementNode::Image variant");
    };
    assert_eq!(props.width, 160.0);
    assert_eq!(props.height, 90.0);
    assert!(matches!(
        props.position,
        Some(skia_engine_core::nodes::Position::Relative)
    ));
    assert_eq!(props.x, Some(8.0));
    assert_eq!(props.y, Some(12.0));
    assert!(matches!(
        props.vertical_alignment,
        Some(skia_engine_core::nodes::VerticalAlignment::Top)
    ));
    assert!(matches!(
        props.horizontal_alignment,
        Some(skia_engine_core::nodes::HorizontalAlignment::Left)
    ));
    assert_eq!(props.image_src.as_deref(), Some("assets/photo.png"));
    assert_eq!(props.image_sampling, Some(sampling));
    assert_eq!(props.background_color, Some(Color::WHITE));
    assert_eq!(props.border_radius, Some(6.0));
}

#[test]
fn test_skia_rsx_image_node() {
    let sampling = skia_engine_core::nodes::SamplingOptions::new(
        skia_safe::FilterMode::Linear,
        skia_safe::MipmapMode::None,
    );
    let node: ElementNode = skia_rsx! {
        <Image
            width={160.0}
            height={90.0}
            position=skia_engine_core::nodes::Position::Relative
            x={8.0}
            y={12.0}
            vertical_alignment=skia_engine_core::nodes::VerticalAlignment::Top
            horizontal_alignment=skia_engine_core::nodes::HorizontalAlignment::Left
            image_src="assets/photo.png"
            image_sampling=sampling
            background_color=Color::WHITE
            border_radius={6.0}
        />
    };

    let ElementNode::Image { props } = node else {
        panic!("Expected an ElementNode::Image variant");
    };
    assert_eq!(props.width, 160.0);
    assert_eq!(props.height, 90.0);
    assert!(matches!(
        props.position,
        Some(skia_engine_core::nodes::Position::Relative)
    ));
    assert_eq!(props.x, Some(8.0));
    assert_eq!(props.y, Some(12.0));
    assert!(matches!(
        props.vertical_alignment,
        Some(skia_engine_core::nodes::VerticalAlignment::Top)
    ));
    assert!(matches!(
        props.horizontal_alignment,
        Some(skia_engine_core::nodes::HorizontalAlignment::Left)
    ));
    assert_eq!(props.image_src.as_deref(), Some("assets/photo.png"));
    assert_eq!(props.image_sampling, Some(sampling));
    assert_eq!(props.background_color, Some(Color::WHITE));
    assert_eq!(props.border_radius, Some(6.0));
}
