use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Node {
    pub freq: usize,
    pub symbol: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub fn build_frequency_map(text: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();
    //counts occurrences of each char
    for c in text.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }
    //println!("{:?}",freq_map);
    freq_map
}
pub fn build_huffman_tree(freq_map: &HashMap<char, usize>) -> Node {
    //basically, a binary heap is a pre-built btree
    let mut heap = BinaryHeap::new();
    //create every leaf node for all the values from hmap
    for (&symbol, &freq) in freq_map {
        heap.push(Node {
            freq,
            symbol: Some(symbol),
            left: None,
            right: None,
        });
    }
    //get two smallest nodes from heap, make them into one, and push it
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let freq = left.freq + right.freq;
        let node = Node {
            freq,
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        heap.push(node);
    }
    heap.pop().unwrap()
}

pub fn build_codebook(node: &Node, prefix: String, codebook: &mut HashMap<char, String>) {
    if let Some(symbol) = node.symbol {
        codebook.insert(symbol, prefix);
    } else {
        if let Some(ref left) = node.left {
            build_codebook(left, prefix.clone() + "0", codebook);
        }
        if let Some(ref right) = node.right {
            build_codebook(right, prefix + "1", codebook);
        }
    }
}

pub fn encode(text: &str, codebook: &HashMap<char, String>) -> String {
    let mut encoded = String::new();
    for c in text.chars() {
        encoded += codebook.get(&c).unwrap();
    }
    encoded
}

pub fn decode(encoded: &str, root: &Node) -> String {
    let mut decoded = String::new();
    let mut node = root;
    for bit in encoded.chars() {
        node = if bit == '0' {
            node.left.as_ref().unwrap()
        } else {
            node.right.as_ref().unwrap()
        };
        if node.symbol.is_some() {
            decoded.push(node.symbol.unwrap());
            node = root;
        }
    }
    decoded
}