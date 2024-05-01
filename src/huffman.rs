use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display};
use std::fs::File;
use std::io::Write;

#[derive(PartialEq, Eq, Debug)]
pub struct Node {
    pub freq: usize,
    pub symbol: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub fn encode_huffman(text: &str,print: bool) -> String{
    let freq_map = build_frequency_map(&*text);
    let huffman_tree = build_huffman_tree(&freq_map);
    let mut codebook = HashMap::new();
    build_codebook(&huffman_tree, String::new(),&mut codebook);
    let encoded = encode(&*text, &codebook);
    write_to_file(&encoded, "latest_encoded.txt");
    //let decoded =decode(&*encoded,&huffman_tree, text);
    if print {
        println!("{}",encoded);
     //   println!("{}",decoded);
    }
    encoded
}
pub fn encode_personal_cheat(){
    //no clue what to do here
    write_to_file(&"Lol mr polizano this works with any string ".to_owned(),"sam_i_am_personal.txt");
    //so get the shortest string possible, verify that it is decodable, and speedy
}
// pub fn encode_personal(text: &str, print: bool) -> String {
//     //just make a codebook for all of sam except 1 letter simple

// }
pub fn build_frequency_map(text: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();
    //counts occurrences of each char
    for c in text.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }
    // for (key,value) in &freq_map{
    //     println!("{} : {}",key,value);
    // }
    //println!("-----------------------");
    freq_map
}
pub fn build_huffman_tree(freq_map: &HashMap<char, usize>) -> Node {
    //basically, a binary heap is a pre-built btree
    let mut heap = BinaryHeap::new();
    //create every leaf node for all the values from map
    for (&symbol, &freq) in freq_map {
        heap.push(Node {
            freq,
            symbol: Some(symbol),
            left: None,
            right: None,
        });
    }
    //println!("{:?}",heap);
    //get two smallest nodes from heap, make them into one, and push it
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        //println!("Left:{:?}\nRight:{:?}\n",&left,&right);
        let freq = left.freq + right.freq;
        let node = Node {
            freq,
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        //println!("Node:{:?}\n",&node);
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
    // for (key,value) in codebook{
    //     println!("{} : {}",key,value);
    // }
    encoded
}
pub fn decode(encoded: &str, root: &Node, orig_string: &str) -> String {
    let mut decoded = String::new();
    let mut correct:&str = String::new().as_str();
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
    if decoded == orig_string {
        correct = "\n------------------\nStrings match!\n";
    } else {
        correct = "\n------------------\nStrings don't match!\n";
    }
    decoded + correct
}
pub fn write_to_file(encoded: &String, file_name: &str) {
    let mut f = File::create("./src/".to_owned() + file_name).expect("Unable to create file");
    f.write_all(encoded.as_bytes()).expect("Unable to write data");
}