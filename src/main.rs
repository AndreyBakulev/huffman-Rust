pub mod huffman;
pub mod shannon_fano;
use huffman::*;
use shannon_fano::*;
use std::fs;
use std::io::stdin;
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Welcome to Andrey's Huffman and Shannon-Fano Encoder!");
    println!("Enter text to encode, give a file directory, or type 'exit' to quit:");
    loop {
        let mut text = String::new();
        if text.trim() == "exit" {
            println!("Goodbye!");
            break;
        }
        stdin().read_line(&mut text).expect("Failed to read line");
        if text.trim() == "cheat" {
            println!("Hehhehehehe you have decided to cheat");
            encode_personal_cheat();
            let mut text3 = String::new();
            println!("Would you like to decode the string?");
            stdin().read_line(&mut text3).expect("Failed to read line");
            if text3.trim() == "yes" || text3.trim() == "1" || text3.trim() == "true" {
                let timer2 = Instant::now();
                decode_latest(None);
                println!("Took {:?} to decode a string of len 2", timer2.elapsed());
            }
        } else {
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
            println!("Select the encoding algorithm:");
            println!("1. Huffman coding");
            println!("2. Shannon-Fano coding");
            let mut choice = String::new();
            stdin().read_line(&mut choice).expect("Failed to read line");
            let choice: u32 = choice.trim().parse().expect("Invalid choice");
            match choice {
                1 => {
                    // Huffman coding
                    let (encoded, latest_root) = encode_huffman(&*text, print);
                    let mut text3 = String::new();
                    println!("Would you like to decode the string?");
                    stdin().read_line(&mut text3).expect("Failed to read line");
                    let timer2 = Instant::now();
                    if text3.trim() == "yes" || text3.trim() == "1" || text3.trim() == "true" {
                        decode_latest(Some(&latest_root));
                        let time_taken2 = timer2.elapsed();
                        println!("Took {:?} to decode a string of len {}", time_taken2, encoded.len());
                    }
                }
                2 => {
                    // Shannon-Fano coding
                    let (encoded, codebook) = encode_shannon_fano(&*text);
                    write_to_file(&encoded, "latest_encoded.txt");
                    if print {
                        println!("{}", encoded);
                    }
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
                        let decoded = decode_shannon_fano(&encoded, &codebook);
                        println!("Decoded string: {}", decoded);
                        let time_taken2 = timer2.elapsed();
                        println!("Took {:?} to decode a string of len {}", time_taken2, encoded.len());
                    }
                }
                _ => println!("Invalid choice"),
            }
        }
    }
}