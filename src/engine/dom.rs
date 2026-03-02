/**
 * @file: dom.rs
 * @author: Krisna Pranav
*/

#[derive(Debug, Clone)]
pub enum NodeType {
    Text(String),
    Element(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            children: Vec::new(),
        }
    }
}