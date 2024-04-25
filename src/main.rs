pub mod huffman;
pub use crate::huffman::*;
use huffman::*;
use std::collections::HashMap;
use std::io::stdin;
use std::time::Instant;

fn main() {
    println!("Welcome to Andrey's Huffman Encoder! Enter text to encode:");
    // let mut text = String::new();
    // stdin().read_line(&mut text).expect("Failed to read line");
    let text = "I AM SAM. I AM SAM. SAM I AM.

THAT SAM-I-AM! THAT SAM-I-AM! I DO NOT LIKE THAT SAM-I-AM!

DO WOULD YOU LIKE GREEN EGGS AND HAM?

I DO NOT LIKE THEM,SAM-I-AM.
I DO NOT LIKE GREEN EGGS AND HAM.

WOULD YOU LIKE THEM HERE OR THERE?

I WOULD NOT LIKE THEM HERE OR THERE.
I WOULD NOT LIKE THEM ANYWHERE.
I DO NOT LIKE GREEN EGGS AND HAM.
I DO NOT LIKE THEM, SAM-I-AM.";
    let freq_map = build_frequency_map(&*text);
    let huffman_tree = build_huffman_tree(&freq_map);
    let mut codebook = HashMap::new();
    build_codebook(&huffman_tree, String::new(),&mut codebook);
    let timer = Instant::now();
    let encoded = encode(&*text, &codebook);
    let decoded = decode(&encoded, &huffman_tree);
    let time_taken = timer.elapsed();
    println!("Encoded text: {}", encoded);
    println!("Decoded text: {}", decoded);
    println!("Took {:?} to encode + decode a string of len {}",time_taken, text.len());
}
/*
PROBLEMS:
I think I made my map work backwards so the most common characters r the longest which is obv bad
 */