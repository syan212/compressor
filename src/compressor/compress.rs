use crate::compressor::node::Node;
use std::collections::{BinaryHeap, HashMap};

fn build_huffman_tree(freq_map: &HashMap<u8, u32>) -> Option<Node> {
    let mut priority_queue: BinaryHeap<Node> = BinaryHeap::new();

    // Create leaf nodes for each byte and add to the heap
    for (&byte, &freq) in freq_map {
        priority_queue.push(Node::Leaf { byte, freq });
    }
    if priority_queue.is_empty() {
        return None;
    }
    if priority_queue.len() == 1 {
        // Edge case: single symbol, use length 1
        let node = priority_queue.pop().unwrap();
        return Some(Node::Internal {
            left: Box::new(node),
            right: Box::new(Node::Leaf { byte: 0, freq: 0 }),
            freq: 0,
        });
    }

    // Build the tree
    while priority_queue.len() > 1 {
        let left = priority_queue.pop().unwrap();
        let right = priority_queue.pop().unwrap();
        let merged_freq = left.freq() + right.freq();
        let internal_node = Node::Internal {
            left: Box::new(left),
            right: Box::new(right),
            freq: merged_freq,
        };
        priority_queue.push(internal_node);
    }

    priority_queue.pop()
}

pub fn compress(data: Vec<u8>) -> Option<Node> {
    let freq_map = super::freq::freq_analysis(data.clone());
    let huffman_tree = build_huffman_tree(&freq_map)?;

    // TODO: Implement actual compression logic using the Huffman tree
    Some(huffman_tree) // Return tree for now
}
