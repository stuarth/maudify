use std::io::{self, Read};
use visdom::Vis;

fn node_to_maud(node: visdom::types::BoxDynElement, indent_level: usize) -> String {
    let indent = "    ".repeat(indent_level);

    use visdom::types::INodeType::*;

    match node.node_type() {
        Comment | HTMLDOCTYPE => String::new(),
        Element => {
            let tag_name = node.tag_name().to_lowercase();
            let mut attrs = String::new();
            for (name, value) in node.get_attributes() {
                attrs.push_str(&format!(" {}=\"{}\"", name, value.to_list().join(" ")));
            }

            let mut children_content = Vec::new();
            for child in node.children() {
                let child_maud = node_to_maud(child, indent_level + 1);
                if !child_maud.trim().is_empty() {
                    children_content.push(child_maud);
                }
            }

            if children_content.is_empty() {
                match node.text() {
                    s if s.is_empty() => format!("{}{}{} {{}}", indent, tag_name, attrs),
                    s => format!("{}{}{} {{ \"{}\" }}", indent, tag_name, attrs, s),
                }
            } else {
                let children = children_content.join("\n");
                format!(
                    "{}{}{} {{\n{}\n{}}}",
                    indent, tag_name, attrs, children, indent
                )
            }
        }
        _ => String::new(),
    }
}

fn main() {
    let mut html = String::new();
    io::stdin()
        .read_to_string(&mut html)
        .expect("Error reading from stdin");

    let root = Vis::load(html.as_str()).unwrap();
    for node in root.children("") {
        let maud_syntax = node_to_maud(node, 0);
        if !maud_syntax.trim().is_empty() {
            println!("{}", maud_syntax);
        }
    }
}
