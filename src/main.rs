mod huffman;

use huffman::*;
use std::collections::HashMap;
use std::io::stdin;

fn main() {
    println!("Welcome to Andrey's Huffman Encoder! Enter text to encode:");
    let mut text = String::new();
    stdin().read_line(&mut text).expect("Failed to read line");
    let freq_map = build_frequency_map(&*text);
    let huffman_tree = build_huffman_tree(&freq_map);
    let mut codebook = HashMap::new();
    build_codebook(&huffman_tree, String::new(), &mut codebook);

    let encoded = encode(&*text, &codebook);
    let decoded = decode(&encoded, &huffman_tree);
    println!("Encoded text: {}", encoded);
    println!("Decoded text: {}", decoded);
}
/*
HOW TO:
get total amt of each char in a given string
combine two least seen char's into a string "c + b -> cb | value
make a dict for each char and occurences
now do it again (combine two least common) e + cb -> ecb | value
do this until you have one string
this should be a binary tree of the single characters

each time there is a left, type 0
each time right, type 1
trace a path using 1's and 0's to each char (c = 1110)
concat back into string of 0's and 1's
 */