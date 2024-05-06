pub mod huffman;
mod image;
pub use crate::huffman::*;
use huffman::*;
use std::fs;
use std::io::stdin;
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Welcome to Andrey's Huffman Encoder! Enter text to encode,give a file directory, or type 'exit' to quit:");
    loop {
        let mut text = String::new();
        if text.trim() == "exit" {
            println!("Goodbye!");
            break;
        }
        stdin().read_line(&mut text).expect("Failed to read line");
        if text.trim() == "cheat" {
            println!("Hehhehehehe you have decided to cheat");
            encode_personal_cheat(&*text);
        }
        let path = "./src/".to_owned() + &*text.clone().trim();
        if Path::new(&path).exists() {
            text = fs::read_to_string(&path).expect("Should have been able to read the file");
        } else {
            println!("No path found, encoding input");
        }
        let mut text2 = String::new();
        println!("Would you like to print out the encoded string?");
        stdin().read_line(&mut text2).expect("Failed to read line");
        let mut print = false;
        if text2.trim() == "yes" || text2.trim() == "1" || text2.trim() == "true" {
            print = true;
        }
        let timer = Instant::now();
        let (encoded,latest_root) = encode_huffman(&*text,print);
        let time_taken = timer.elapsed();
        println!(
            "Took {:?} to encode a string of len {}",
            time_taken,
            text.len()
        );
        let percentage_cut: f32 = (encoded.len() as f32 / ((text.len() as f32) * 8f32)) * 100f32;
        println!(
            "Cut down from {} to {} bits! ({}%)",
            text.len() * 8,
            encoded.len(),
            percentage_cut
        );
        let mut text3 = String::new();
        println!("Would you like to decode the string?");
        stdin().read_line(&mut text3).expect("Failed to read line");
        let timer2 = Instant::now();
        if text3.trim() == "yes" || text3.trim() == "1" || text3.trim() == "true" {
            decode_latest(&latest_root);
            let time_taken2 = timer2.elapsed();
            println!(
                "Took {:?} to decode a string of len {}",
                time_taken2,
                encoded.len()
            );
        }
    }
}
/*

TODO:
write the encoded string to file and feed it into decompression
PROBLEMS:
    none :D

Additions:
shannon-fano coding
DEFLATE
LZ4

Add compression for images + audio

end goal is to zip files with this algorithm
 */
