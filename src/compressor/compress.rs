use crate::compressor::node::Node;
use std::collections::{BinaryHeap, HashMap};

fn build_huffman_tree(freq_map: &HashMap<u8, u32>) -> Option<Node> {
    let mut heap = BinaryHeap::new();

    // Create leaf nodes for each byte and add to the heap
    for (&byte, &freq) in freq_map {
        heap.push(Node::Leaf { byte, freq });
    }
    if heap.is_empty() { return None; }
    if heap.len() == 1 {
        // Edge case: single symbol → use length 1
        let node = heap.pop().unwrap();
        return Some(Node::Internal { left: Box::new(node), right: Box::new(Node::Leaf { byte: 0, freq: 0 }), freq: 0 });
    }


    // Build the tree
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let merged_freq = left.freq() + right.freq();
        let internal_node = Node::Internal {
            left: Box::new(left),
            right: Box::new(right),
            freq: merged_freq,
        };
        heap.push(internal_node);
    }

    heap.pop()
}

pub fn compress(data: Vec<u8>) -> Option<Node> {
    let freq_map = super::freq::freq_analysis(data.clone());
    let huffman_tree = build_huffman_tree(&freq_map)?;

    // TODO: Implement actual compression logic using the Huffman tree
    Some(huffman_tree) // Return original data for now
}