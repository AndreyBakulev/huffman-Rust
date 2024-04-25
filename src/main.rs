pub mod huffman;
pub use crate::huffman::*;
use huffman::*;
use std::collections::HashMap;
use std::fs;
use std::io::stdin;
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Welcome to Andrey's Huffman Encoder! Enter text to encode or give a file directory!:");
    let mut text = String::new();
    stdin().read_line(&mut text).expect("Failed to read line");
    let path = "./src/".to_owned() + &*text.clone().trim();
    if Path::new(&path).exists(){
        text = fs::read_to_string(&path)
            .expect("Should have been able to read the file");
    } else {
        println!("No path found, encoding input");
    }
    let freq_map = build_frequency_map(&*text);
    let huffman_tree = build_huffman_tree(&freq_map);
    let mut codebook = HashMap::new();
    build_codebook(&huffman_tree, String::new(),&mut codebook);
    let timer = Instant::now();
    let encoded = encode(&*text, &codebook);
    let decoded = decode(&encoded, &huffman_tree);
    let time_taken = timer.elapsed();
    //println!("Encoded text: {}", encoded);
    //println!("Decoded text: {}", decoded);
    println!("Took {:?} to encode + decode a string of len {}",time_taken, text.len());
    let percentage_cut:f32 = (encoded.len() as f32/((text.len() as f32)*8f32))* 100f32;
    println!("Cut down from {} to {} bits! ({}%)",text.len()*8,encoded.len(),percentage_cut);
}
/*
PROBLEMS:
    none :D

Additions:
shannon-fano coding
DEFLATE
LZ4

Add compression for images + audio

end goal is to zip files with this algorithm
 */