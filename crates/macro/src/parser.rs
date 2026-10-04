//! Parser for the nested brace syntax accepted by [`crate::skia_ui`].
//!
//! An element consists of an identifier followed by a braced body. Body
//! entries are either `name: expression` attributes or nested `Tag { ... }`
//! elements, separated by commas.
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Result, Token};

/// Parsed element in the brace-based UI syntax.
pub struct SkiaNodeInput {
    /// Element tag, such as `Container`, `Rect`, `Image`, or `Text`.
    pub tag_name: Ident,
    /// Properties written as `name: expression`.
    pub attributes: Vec<Attribute>,
    /// Nested child elements.
    pub children: Vec<SkiaNodeInput>,
}

/// Parsed property name and Rust expression value.
pub struct Attribute {
    /// Property identifier.
    pub key: Ident,
    /// Property value expression.
    pub value: Expr,
}

impl Parse for SkiaNodeInput {
    /// Parses one root element and its braced body.
    fn parse(input: ParseStream) -> Result<Self> {
        let tag_name = input.parse()?;
        let content;
        syn::braced!(content in input);
        parse_node_body(tag_name, &content)
    }
}

/// Parses the contents of an element body recursively.
///
/// Entries must be attributes or child elements. A comma is required between
/// entries, though the final entry may omit its trailing comma.
fn parse_node_body(tag_name: Ident, content: ParseStream) -> Result<SkiaNodeInput> {
    let mut attributes = Vec::new();
    let mut children = Vec::new();

    while !content.is_empty() {
        let name: Ident = content.parse()?;
        if content.peek(Token![:]) {
            content.parse::<Token![:]>()?;
            let value = content.parse()?;
            attributes.push(Attribute { key: name, value });
        } else if content.peek(syn::token::Brace) {
            let child_content;
            syn::braced!(child_content in content);
            children.push(parse_node_body(name, &child_content)?);
        } else {
            return Err(content.error("Expected `:` for an attribute or `{` for a child element"));
        }

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        } else if !content.is_empty() {
            return Err(content.error("Expected `,` between attributes and child elements"));
        }
    }

    Ok(SkiaNodeInput {
        tag_name,
        attributes,
        children,
    })
}
