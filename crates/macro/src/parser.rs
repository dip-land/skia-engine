use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Result, Token};

pub struct SkiaNodeInput {
    pub tag_name: Ident,
    pub attributes: Vec<Attribute>,
    pub children: Vec<SkiaNodeInput>,
}

pub struct Attribute {
    pub key: Ident,
    pub value: Expr,
}

impl Parse for SkiaNodeInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag_name = input.parse()?;
        let content;
        syn::braced!(content in input);
        parse_node_body(tag_name, &content)
    }
}

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
