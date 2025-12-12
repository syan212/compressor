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

fn generate_huffman_codes(
    node: &Node,
    prefix: Vec<bool>,
    codes: &mut HashMap<u8, Vec<bool>>,
) {
    match node {
        Node::Leaf { byte, .. } => {
            codes.insert(*byte, prefix);
        }
        Node::Internal { left, right, .. } => {
            let mut left_prefix = prefix.clone();
            left_prefix.push(false);
            generate_huffman_codes(left, left_prefix, codes);
            let mut right_prefix = prefix;
            right_prefix.push(true);
            generate_huffman_codes(right, right_prefix, codes);
        }
    }
}

pub fn compress(data: Vec<u8>) -> Option<Vec<u8>> {
    let freq_map = super::freq::freq_analysis(data.clone());
    let huffman_tree = build_huffman_tree(&freq_map);
    let mut huffman_codes: HashMap<u8, Vec<bool>> = HashMap::new();
    match &huffman_tree {
        None => return None,
        Some(tree) => {
            generate_huffman_codes(tree, Vec::new(), &mut huffman_codes);
            let mut compressed_data: Vec<u8> = Vec::new();
            let mut current_byte: u8 = 0;
            let mut bit_count: u8 = 0;
            println!("Compressing data");
            for byte in data {
                let code = huffman_codes.get(&byte).unwrap();
                for bit in code {
                    current_byte <<= 1;
                    bit_count += 1;
                    if *bit {
                        current_byte |= 1;
                    }
                    if bit_count == 8 {
                        compressed_data.push(current_byte);
                        current_byte = 0;
                        bit_count = 0;
                    }
                }
            }
            // Push remaining bits if any
            if bit_count > 0 {
                compressed_data.push(current_byte << (8 - bit_count));
            }
            Some(compressed_data)
        }
    }
}
