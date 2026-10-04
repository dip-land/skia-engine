//! Procedural macros for constructing `skia-engine` element trees.
//!
//! [`skia_ui!`] provides a brace-based syntax, while [`skia_rsx!`] accepts
//! RSX-like tags. Both macros produce `skia_engine_core::ElementNode` values
//! and expect the generated code's `skia_engine_core` and `skia_safe` paths to
//! be available to the consuming crate.
//!
//! Supported elements are `Container`, `Rect`, `Image`, and `Text`.
//! Containers may contain nested elements; the other elements are leaves.
//! Element properties use Rust expressions. `Text` content can be supplied
//! through its `content` property, or as a quoted string literal in the RSX
//! body.
//!
//! # Brace syntax
//!
//! ```ignore
//! skia_ui! {
//!     Container {
//!         width: 320.0,
//!         height: 180.0,
//!         background_color: skia_safe::Color::WHITE,
//!         Rect {
//!             width: 80.0,
//!             height: 60.0,
//!             color: skia_safe::Color::BLUE,
//!         }
//!     }
//! }
//! ```
//!
//! # RSX syntax
//!
//! ```ignore
//! skia_rsx! {
//!     <Container width={320.0} height={180.0}>
//!         <Text font_size={24.0}>{"Hello, Skia!"}</Text>
//!     </Container>
//! }
//! ```
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn_rsx::parse2;

mod parser;

/// A normalized element property used by both macro syntaxes.
struct AttributeTokens {
    /// Property identifier as written in the invocation.
    name: String,
    /// Rust expression to use as the property's value.
    value: proc_macro2::TokenStream,
}

/// Builds an `ElementNode` tree using nested Rust-like brace syntax.
///
/// Each element is written as `Tag { ... }`. Properties use `name: expression`
/// syntax and nested elements are written directly inside their parent's
/// braces. Separate entries with commas. `Container` accepts children;
/// `Rect`, `Image`, and `Text` do not. Text content is provided with the
/// `content` property.
///
/// Supported tags are `Container`, `Rect`, `Image`, and `Text`. Supported
/// properties are validated at compile time; an unknown tag or property, or an
/// invalid element structure, produces a compile error.
///
/// The expansion refers to `skia_engine_core::ElementNode` and, for the
/// default rectangle color, `skia_safe::Color::BLACK`. Those crate paths must
/// be available in the consuming crate.
///
/// # Example
///
/// ```ignore
/// let tree = skia_ui! {
///     Container {
///         width: 200.0,
///         height: 100.0,
///         Rect {
///             width: 50.0,
///             height: 50.0,
///             color: skia_safe::Color::GREEN,
///         }
///     }
/// };
/// ```
#[proc_macro]
pub fn skia_ui(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as parser::SkiaNodeInput);
    expand_skia_ui_node(&root).into()
}

/// Recursively converts a parsed brace-syntax node into an element expression.
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

/// Builds an `ElementNode` tree using RSX-like tag syntax.
///
/// Attribute values may be Rust expressions in braces, such as
/// `width={200.0}`, or Rust paths/identifiers where accepted by the RSX parser.
/// Container tags may nest other elements. `Rect`, `Image`, and `Text` are
/// leaves. Text content must be a string literal body (for example,
/// `<Text>{"Hello"}</Text>`) or supplied with the `content` property, but not
/// both.
///
/// Exactly one root element is required. Unknown tags and properties,
/// valueless attributes, invalid children, and invalid text bodies produce
/// compile-time errors.
///
/// The expansion refers to `skia_engine_core::ElementNode` and, for the
/// default rectangle color, `skia_safe::Color::BLACK`. Those crate paths must
/// be available in the consuming crate.
///
/// # Example
///
/// ```ignore
/// let tree = skia_rsx! {
///     <Container width={200.0} height={100.0}>
///         <Text font_size={20.0}>{"Hello"}</Text>
///     </Container>
/// };
/// ```
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

/// Expands a parsed RSX node, collecting its properties, element children, and
/// (for `Text`) literal body content.
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

/// Validates common element properties and dispatches to the tag-specific
/// element expander.
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
        "Image" => expand_image(properties, children),
        "Text" => expand_text(properties, children, text_content),
        _ => syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Unknown element tag: {tag_name}"),
        )
        .to_compile_error(),
    }
}

/// Concatenates literal text children for an RSX `Text` element.
///
/// Comments are ignored. Non-string text expressions and child elements are
/// rejected, since this macro only supports literal text content.
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

/// Parsed property values shared by the supported element expanders.
///
/// Values remain token streams until an expander emits them in the generated
/// `Props` initializer, preserving Rust expression semantics.
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
    image_src: Option<proc_macro2::TokenStream>,
    image_sampling: Option<proc_macro2::TokenStream>,
}

impl ElementProperties {
    /// Maps property names to their internal slots, rejecting unknown names.
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
                "image_src" => &mut properties.image_src,
                "image_sampling" => &mut properties.image_sampling,
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

/// Emits a container node with recursively expanded children.
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

/// Emits a leaf rectangle node, rejecting any element children.
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

/// Emits a leaf image node, rejecting any element children.
fn expand_image(
    properties: ElementProperties,
    children: Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    if !children.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Image elements cannot contain children",
        )
        .to_compile_error();
    }

    let mut fields = Vec::new();
    if let Some(value) = properties.width {
        fields.push(quote! { width: (#value) as f32 });
    }
    if let Some(value) = properties.height {
        fields.push(quote! { height: (#value) as f32 });
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
    if let Some(value) = properties.image_src {
        fields.push(quote! { image_src: Some((#value).into()) });
    }
    if let Some(value) = properties.image_sampling {
        fields.push(quote! { image_sampling: Some(#value) });
    }
    if let Some(value) = properties.background_color {
        fields.push(quote! { background_color: Some(#value) });
    }
    if let Some(value) = properties.border_radius {
        fields.push(quote! { border_radius: Some((#value) as f32) });
    }

    quote! {
        skia_engine_core::ElementNode::Image {
            props: skia_engine_core::nodes::image::Props {
                #(#fields,)*
                ..Default::default()
            }
        }
    }
}

/// Emits a leaf text node after validating its content source and properties.
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
