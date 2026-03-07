/**
 * @file: html.rs
 * @author: Krisna Pranav
*/

use super::dom::{Node, NodeType};

pub fn parse(source: &str) -> Node {
    let root = Node::new(NodeType::Element("html".into()));
    let mut stack: Vec<Node> = Vec::new();
    stack.push(root);

    let mut text_buffer = String::new();
    let mut inside_tag = false;
    let mut tag_buffer = String::new();

    for c in source.chars() {
        if c == '<' {
            inside_tag = true;

            if !text_buffer.trim().is_empty() {
                let text_node = Node::new(NodeType::Text(text_buffer.clone()));
                stack.last_mut().unwrap().children.push(text_node);
            }

            text_buffer.clear();
        } else if c == '>' {
            inside_tag = false;

            if tag_buffer.starts_with('/') {
                stack.pop();
            } else {
                let new_node = Node::new(NodeType::Element(tag_buffer.clone()));
                stack.last_mut().unwrap().children.push(new_node.clone());
                stack.push(new_node);
            }

            tag_buffer.clear();
        } else {
            if inside_tag {
                tag_buffer.push(c);
            } else {
                text_buffer.push(c);
            }
        }
    }

    stack.first().unwrap().clone()
}