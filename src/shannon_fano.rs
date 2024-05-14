use std::collections::HashMap;

pub fn encode_shannon_fano(text: &str) -> (String, HashMap<char, String>) {
    let freq_map = build_frequency_map(text);
    let mut codebook = HashMap::new();
    let mut sorted_chars: Vec<char> = freq_map.keys().cloned().collect();
    sorted_chars.sort_by(|a, b| freq_map[b].cmp(&freq_map[a]));
    build_shannon_fano_codebook(&sorted_chars, &freq_map, &mut codebook, 0, sorted_chars.len());
    let encoded = encode(text, &codebook);
    (encoded, codebook)
}

fn build_frequency_map(text: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();
    for c in text.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }
    freq_map
}

fn build_shannon_fano_codebook(
    sorted_chars: &[char],
    freq_map: &HashMap<char, usize>,
    codebook: &mut HashMap<char, String>,
    start: usize,
    end: usize,
) {
    if start >= end {
        return;
    }
    if end - start == 1 {
        codebook.insert(sorted_chars[start], "0".to_string());
        return;
    }
    let mid = find_split_index(sorted_chars, freq_map, start, end);
    for i in start..mid {
        let symbol = sorted_chars[i];
        let code = codebook.entry(symbol).or_insert_with(String::new);
        code.push('0');
    }
    for i in mid..end {
        let symbol = sorted_chars[i];
        let code = codebook.entry(symbol).or_insert_with(String::new);
        code.push('1');
    }
    build_shannon_fano_codebook(sorted_chars, freq_map, codebook, start, mid);
    build_shannon_fano_codebook(sorted_chars, freq_map, codebook, mid, end);
}

fn find_split_index(
    sorted_chars: &[char],
    freq_map: &HashMap<char, usize>,
    start: usize,
    end: usize,
) -> usize {
    let total_freq: usize = sorted_chars[start..end].iter().map(|c| freq_map[c]).sum();
    let mut left_freq = 0;
    let mut index = start;
    while index < end {
        left_freq += freq_map[&sorted_chars[index]];
        if left_freq >= (total_freq - left_freq) {
            break;
        }
        index += 1;
    }
    index
}

fn encode(text: &str, codebook: &HashMap<char, String>) -> String {
    let mut encoded = String::new();
    for c in text.chars() {
        encoded += codebook.get(&c).unwrap();
    }
    encoded
}

pub fn decode_shannon_fano(encoded: &str, codebook: &HashMap<char, String>) -> String {
    let mut decoded = String::new();
    let mut current_code = String::new();
    for bit in encoded.chars() {
        current_code.push(bit);
        for (symbol, code) in codebook {
            if code == &current_code {
                decoded.push(*symbol);
                current_code.clear();
                break;
            }
        }
    }
    decoded
}

pub fn write_to_file1(encoded: &str, file_name: &str) {
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(encoded.as_bytes())
        .expect("Unable to write data to file");
}