/**
 * @file: layout.rs
 * @author: Krisna Pranav
*/

use super::dom::{Node, NodeType};
#[derive(Clone)]
pub struct LayoutBox {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub fn build_layout(dom: &Node, screen_width: usize) -> Vec<LayoutBox> {
    let mut boxes = Vec::new();
    let mut current_y = 10;

    build_node(dom, screen_width, &mut current_y, &mut boxes);

    boxes
}

fn build_node(
    node: &Node,
    screen_width: usize,
    current_y: &mut usize,
    boxes: &mut Vec<LayoutBox>,
) {
    match &node.node_type {
        NodeType::Text(text) => {
            let height = 20;
            boxes.push(LayoutBox {
                x: 10,
                y: *current_y,
                width: screen_width - 20,
                height,
            });
            *current_y += height + 5;
        }
        NodeType::Element(_) => {
            for child in &node.children {
                build_node(child, screen_width, current_y, boxes);
            }
        }
    }
}