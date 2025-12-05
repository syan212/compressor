use std::cmp::Ordering;

#[derive(Debug)]
pub enum Node {
    // Leaf node
    Leaf { byte: u8, freq: u32 },
    // Internal node, with left and right children
    Internal { left: Box<Node>, right: Box<Node>, freq: u32 },
}

// Helper method to get frequency of a node
impl Node {
    pub fn freq(&self) -> u32 {
        match self {
            Node::Leaf { freq, .. } => *freq,
            Node::Internal { freq, .. } => *freq,
        }
    }
}

// Implement ordering traits for Node based on frequency
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq().cmp(&self.freq()) // reversed for min-heap
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl PartialEq for Node {
    fn eq(&self, o: &Self) -> bool {
        self.freq() == o.freq()
    }
}
impl Eq for Node {}
