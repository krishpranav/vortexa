/**
 * @file: render.rs
 * @author: Krisna Pranav
*/

use super::dom::{Node, NodeType};

pub fn render(node: &Node) {
    render_node(node, 0);
}

fn render_node(node: &Node, depth: usize) {
    match &node.node_type {
        NodeType::Text(text) => {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                println!("{}", trimmed);
            }
        }
        NodeType::Element(tag) => {
            match tag.as_str() {
                "h1" => println!("\n===================="),
                "p" => println!(),
                "a" => println!("[Link]"),
                _ => {}
            }

            for child in &node.children {
                render_node(child, depth + 1);
            }

            if tag == "h1" {
                println!("====================");
            }
        }
    }
}