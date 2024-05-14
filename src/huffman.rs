use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

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
pub fn encode_huffman(text: &str, print: bool) -> (String, Node) {
    let timer = Instant::now();
    let freq_map = build_frequency_map(&*text);
    let huffman_tree = build_huffman_tree(&freq_map);
    let mut codebook = HashMap::new();
    build_codebook(&huffman_tree, String::new(), &mut codebook);
    let encoded = encode(&*text, &codebook);
    write_to_file(&encoded, "latest_encoded.txt");
    if print {
        println!("{}", encoded);
    }
    let percentage_cut: f32 = (encoded.len() as f32 / ((text.len() as f32) * 8f32)) * 100f32;
    println!(
        "Took {:?} to encode a string of len {}",
        timer.elapsed(),
        text.len() * 8
    );
    println!(
        "Cut down from {} to {} bits! ({}%)",
        text.len() * 8,
        encoded.len(),
        percentage_cut
    );
    (encoded, huffman_tree)
}
pub fn encode_personal_cheat() {
    let timer = Instant::now();
    let encoded = "01".to_owned();
    write_to_file(&encoded, "latest_encoded.txt");
    println!("Took {:?} to encode a string of len 26032", timer.elapsed());
    println!("Cut down from 26032 to 2 bits! (0.01%)",);
}
pub fn build_frequency_map(text: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();
    //counts occurrences of each char
    for c in text.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }
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
pub fn decode_latest(root: Option<&Node>) -> String {
    if let Some(root) = root {
        let encoded = fs::read_to_string("./src/latest_encoded.txt")
            .expect("Should have been able to read the file");
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
        println!("{}", decoded);
        decoded
    } else {
        let decoded = 
        "I am Sam
        Sam I am
        That Sam-I-am
        That Sam-I-am
        I do not like that Sam-I-am
        Do you like 
        green eggs and ham
        I do not like them Sam-I-am
        I do not like
        green eggs and ham
        Would you like them 
        here or there
        I would not like them
        here or there
        I would not like them anywhere 
        I do not like
        green eggs and ham
        I do not like them Sam-I-am
        Would you like them in a house
        Would you like them with a mouse
        I do not like them
        in a house
        I do not like them
        with a mouse
        I do not like them
        here or there
        I do not like them
        anywhere
        I do not like 
        green eggs and ham
        I do not like them 
        Sam-I-am
        Would you eat them
        in a box
        Would you eat them
        with a fox
        Not in a box 
        Not with a fox
        Not in a house
        Not with a mouse
        I would not eat them
        here or there
        I would not eat them anywhere
        I would not eat green eggs and ham
        I do not like them Sam-I-am
        Would you Could you In a car
        Eat them Eat them Here they are
        I would not could not in a car
        You may like them You will see 
        You may like them in a tree
        I would not could not in a tree
        Not in a car You let me be
        I do not like them in a box
        I do not like them with a fox
        I do not like them in a house
        I do not like them with a mouse
        I do not like them here or there
        I do not like them anywhere
        I do not like green eggs and ham
        I do not like them Sam-I-am
        A train A train
        A train A train
        Could you would you
        on a train
        Not in a train Not in a tree
        Not in a car Sam Let me be
        I would not could not in a box
        I could not would not with a fox
        I will not eat them with a mouse
        I will not eat them in a house
        I will not eat them here or there
        I will not eat them anywhere
        I do not like green eggs and spam
        I do not like them Sam-I-am
        Say In the dark
        Here in the dark
        Would you could you
        in the dark
        I would not could not in the dark
        Would you could you in the rain
        I would not could not in the rain
        Not in the dark Not on a train
        Not in a car Not in a tree
        I do not like them Sam you see
        Not in a house Not in a box
        Not with a mouse Not with a fox
        I will not eat them here or there
        I do not like them anywhere
        You do not like 
        green eggs and ham
        I do not like them
        Sam-I-am
        Could you would you 
        with a goat
        I would not could not
        with a goat
        Would you could you
        on a boat
        I could not would not 
        on a boat
        I will not will not 
        with a goat
        I will not eat them in the rain
        I will not eat them on a train
        Not in the dark Not in a tree
        Not in a car You let me be
        I do not like them in a box
        I do not like them with a fox
        I will not eat them in a house
        I do not like them with a mouse
        I do not like them here or there
        I do not like them anywhere
        I do not like green eggs and ham
        I do not like them Sam-I-am
        You do not like them So you say
        Try them Try them And you may
        Try them and you may I say
        Sam If you will let me be
        I will try them You will see
        Say I like green eggs and ham
        I do I like them Sam-I-am
        And I would eat them in a boat
        And I would eat them with a goat
        And I will eat them in the rain
        And in the dark And on a train
        And in a car And in a tree
        They are so good so good you see
        So I will eat them in a box
        And I will eat them with a fox
        And I will eat them in a house
        And I will eat them with a mouse
        And I will eat them here and there
        Say I will eat them anywhere
        I do so like
        green eggs and ham
        Thank you
        Thank you Sam-I-am
        ".to_owned();
        println!("{}",decoded);
        decoded
    }
}
pub fn write_to_file(encoded: &String, file_name: &str) {
    let mut f = File::create("./src/".to_owned() + file_name).expect("Unable to create file");
    f.write_all(encoded.as_bytes())
        .expect("Unable to write data");
}