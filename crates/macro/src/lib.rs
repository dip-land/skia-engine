use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn_rsx::parse2;

mod parser;

struct AttributeTokens {
    name: String,
    value: proc_macro2::TokenStream,
}

#[proc_macro]
pub fn skia_ui(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as parser::SkiaNodeInput);
    expand_skia_ui_node(&root).into()
}

fn expand_skia_ui_node(node: &parser::SkiaNodeInput) -> proc_macro2::TokenStream {
    let attributes = node.attributes.iter().map(|attribute| {
        let value = &attribute.value;
        AttributeTokens {
            name: attribute.key.to_string(),
            value: quote! { #value },
        }
    });
    let children = node.children.iter().map(expand_skia_ui_node).collect();

    expand_element(&node.tag_name.to_string(), attributes, children, None)
}

#[proc_macro]
pub fn skia_rsx(input: TokenStream) -> TokenStream {
    let nodes = match parse2(input.into()) {
        Ok(nodes) => nodes,
        Err(err) => return err.to_compile_error().into(),
    };

    match nodes.as_slice() {
        [root] => expand_skia_rsx_node(root).into(),
        [] => syn::Error::new(proc_macro2::Span::call_site(), "Expected a root element")
            .to_compile_error()
            .into(),
        _ => syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected a single root element",
        )
        .to_compile_error()
        .into(),
    }
}

fn expand_skia_rsx_node(node: &syn_rsx::Node) -> proc_macro2::TokenStream {
    let syn_rsx::Node::Element(element) = node else {
        return quote! {};
    };

    let mut attributes = Vec::new();
    for node in &element.attributes {
        let syn_rsx::Node::Attribute(attribute) = node else {
            continue;
        };
        let Some(value) = attribute.value.as_ref() else {
            return syn::Error::new_spanned(&attribute.key, "Element properties must have a value")
                .to_compile_error();
        };
        let value = value.as_ref();

        attributes.push(AttributeTokens {
            name: attribute.key.to_string(),
            value: quote! { #value },
        });
    }
    let children = element
        .children
        .iter()
        .filter(|node| matches!(node, syn_rsx::Node::Element(_)))
        .map(expand_skia_rsx_node)
        .collect();

    let text_content = if element.name.to_string() == "Text" {
        match extract_text_content(&element.children) {
            Ok(content) => content,
            Err(error) => return error.to_compile_error(),
        }
    } else {
        None
    };

    expand_element(
        &element.name.to_string(),
        attributes,
        children,
        text_content,
    )
}

fn expand_element(
    tag_name: &str,
    attributes: impl IntoIterator<Item = AttributeTokens>,
    children: Vec<proc_macro2::TokenStream>,
    text_content: Option<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let properties = match ElementProperties::parse(attributes) {
        Ok(properties) => properties,
        Err(error) => return error.to_compile_error(),
    };

    match tag_name {
        "Container" => expand_container(properties, children),
        "Rect" => expand_rect(properties, children),
        "Text" => expand_text(properties, children, text_content),
        _ => syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Unknown element tag: {tag_name}"),
        )
        .to_compile_error(),
    }
}

fn extract_text_content(
    children: &[syn_rsx::Node],
) -> syn::Result<Option<proc_macro2::TokenStream>> {
    let mut content = String::new();
    let mut found_text = false;

    for child in children {
        match child {
            syn_rsx::Node::Text(text) => {
                let value = String::try_from(&text.value).map_err(|_| {
                    syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "Text element content must be a string literal",
                    )
                })?;
                content.push_str(&value);
                found_text = true;
            }
            syn_rsx::Node::Comment(_) => {}
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "Text elements may only contain string literal text",
                ));
            }
        }
    }

    Ok(found_text.then(|| {
        let literal = syn::LitStr::new(&content, proc_macro2::Span::call_site());
        quote! { #literal }
    }))
}

#[derive(Default)]
struct ElementProperties {
    width: Option<proc_macro2::TokenStream>,
    height: Option<proc_macro2::TokenStream>,
    position: Option<proc_macro2::TokenStream>,
    x: Option<proc_macro2::TokenStream>,
    y: Option<proc_macro2::TokenStream>,
    vertical_alignment: Option<proc_macro2::TokenStream>,
    horizontal_alignment: Option<proc_macro2::TokenStream>,
    background_color: Option<proc_macro2::TokenStream>,
    border_radius: Option<proc_macro2::TokenStream>,
    content: Option<proc_macro2::TokenStream>,
    font_bytes: Option<proc_macro2::TokenStream>,
    font_size: Option<proc_macro2::TokenStream>,
    max_width: Option<proc_macro2::TokenStream>,
    max_height: Option<proc_macro2::TokenStream>,
    text_wrap: Option<proc_macro2::TokenStream>,
    text_overflow: Option<proc_macro2::TokenStream>,
    text_alignment: Option<proc_macro2::TokenStream>,
}

impl ElementProperties {
    fn parse(attributes: impl IntoIterator<Item = AttributeTokens>) -> syn::Result<Self> {
        let mut properties = Self::default();

        for attribute in attributes {
            let target = match attribute.name.as_str() {
                "width" => &mut properties.width,
                "height" => &mut properties.height,
                "position" => &mut properties.position,
                "x" => &mut properties.x,
                "y" => &mut properties.y,
                "vertical_alignment" => &mut properties.vertical_alignment,
                "horizontal_alignment" => &mut properties.horizontal_alignment,
                "color" | "background_color" => &mut properties.background_color,
                "border_radius" => &mut properties.border_radius,
                "content" => &mut properties.content,
                "font_bytes" => &mut properties.font_bytes,
                "font_size" => &mut properties.font_size,
                "max_width" => &mut properties.max_width,
                "max_height" => &mut properties.max_height,
                "text_wrap" => &mut properties.text_wrap,
                "text_overflow" => &mut properties.text_overflow,
                "text_alignment" => &mut properties.text_alignment,
                other => {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!("Unknown property: {other}"),
                    ));
                }
            };
            *target = Some(attribute.value);
        }

        Ok(properties)
    }
}

fn expand_container(
    properties: ElementProperties,
    children: Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let mut fields = Vec::new();

    if let Some(value) = properties.width {
        fields.push(quote! { width: Some((#value) as f32) });
    }
    if let Some(value) = properties.height {
        fields.push(quote! { height: Some((#value) as f32) });
    }
    if let Some(value) = properties.position {
        fields.push(quote! { position: Some(#value) });
    }
    if let Some(value) = properties.x {
        fields.push(quote! { x: Some((#value) as f32) });
    }
    if let Some(value) = properties.y {
        fields.push(quote! { y: Some((#value) as f32) });
    }
    if let Some(value) = properties.vertical_alignment {
        fields.push(quote! { vertical_alignment: Some(#value) });
    }
    if let Some(value) = properties.horizontal_alignment {
        fields.push(quote! { horizontal_alignment: Some(#value) });
    }
    if let Some(value) = properties.background_color {
        fields.push(quote! { background_color: Some(#value) });
    }
    if let Some(value) = properties.border_radius {
        fields.push(quote! { border_radius: Some((#value) as f32) });
    }

    quote! {
        skia_engine_core::ElementNode::Container {
            props: skia_engine_core::nodes::container::Props {
                #(#fields,)*
                ..Default::default()
            },
            children: vec![#(#children),*],
        }
    }
}

fn expand_rect(
    properties: ElementProperties,
    children: Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    if !children.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Rect elements cannot contain children",
        )
        .to_compile_error();
    }

    let width = properties
        .width
        .map(|value| quote! { (#value) as f32 })
        .unwrap_or_else(|| quote! { 0.0 });
    let height = properties
        .height
        .map(|value| quote! { (#value) as f32 })
        .unwrap_or_else(|| quote! { 0.0 });
    let background_color = properties
        .background_color
        .unwrap_or_else(|| quote! { skia_safe::Color::BLACK });

    let mut fields = Vec::new();
    if let Some(value) = properties.position {
        fields.push(quote! { position: Some(#value) });
    }
    if let Some(value) = properties.x {
        fields.push(quote! { x: Some((#value) as f32) });
    }
    if let Some(value) = properties.y {
        fields.push(quote! { y: Some((#value) as f32) });
    }
    if let Some(value) = properties.vertical_alignment {
        fields.push(quote! { vertical_alignment: Some(#value) });
    }
    if let Some(value) = properties.horizontal_alignment {
        fields.push(quote! { horizontal_alignment: Some(#value) });
    }
    if let Some(value) = properties.border_radius {
        fields.push(quote! { border_radius: Some((#value) as f32) });
    }

    quote! {
        skia_engine_core::ElementNode::Rect {
            props: skia_engine_core::nodes::rect::Props {
                width: #width,
                height: #height,
                background_color: #background_color,
                #(#fields,)*
                ..Default::default()
            }
        }
    }
}

fn expand_text(
    properties: ElementProperties,
    children: Vec<proc_macro2::TokenStream>,
    text_content: Option<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    if !children.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Text elements cannot contain child elements; use the `content` property",
        )
        .to_compile_error();
    }

    if properties.content.is_some() && text_content.is_some() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Specify Text content either as a `content` property or as body text, not both",
        )
        .to_compile_error();
    }

    let Some(content) = properties.content.or(text_content) else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Text elements require a string literal body or a `content` property",
        )
        .to_compile_error();
    };

    let mut fields = Vec::new();
    if let Some(value) = properties.background_color {
        fields.push(quote! { color: Some(#value) });
    }
    if let Some(value) = properties.font_bytes {
        fields.push(quote! { font_bytes: #value });
    }
    if let Some(value) = properties.font_size {
        fields.push(quote! { font_size: Some((#value) as f32) });
    }
    if let Some(value) = properties.max_width {
        fields.push(quote! { max_width: Some((#value) as f32) });
    }
    if let Some(value) = properties.max_height {
        fields.push(quote! { max_height: Some((#value) as f32) });
    }
    if let Some(value) = properties.text_wrap {
        fields.push(quote! { text_wrap: Some(#value) });
    }
    if let Some(value) = properties.text_overflow {
        fields.push(quote! { text_overflow: Some(#value) });
    }
    if let Some(value) = properties.position {
        fields.push(quote! { position: Some(#value) });
    }
    if let Some(value) = properties.x {
        fields.push(quote! { x: Some((#value) as f32) });
    }
    if let Some(value) = properties.y {
        fields.push(quote! { y: Some((#value) as f32) });
    }
    if let Some(value) = properties.vertical_alignment {
        fields.push(quote! { vertical_alignment: Some(#value) });
    }
    if let Some(value) = properties.horizontal_alignment {
        fields.push(quote! { horizontal_alignment: Some(#value) });
    }
    if let Some(value) = properties.text_alignment {
        fields.push(quote! { text_alignment: Some(#value) });
    }

    quote! {
        skia_engine_core::ElementNode::Text {
            props: skia_engine_core::nodes::text::Props {
                #(#fields,)*
                ..Default::default()
            },
            content: #content,
        }
    }
}
