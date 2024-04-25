use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crate::huffman::*;
use std::collections::HashMap;

fn huffman_encoding_benchmark(c: &mut Criterion) {
    let text = "This is a sample text for benchmarking the Huffman coding implementation.";
    let freq_map = build_frequency_map(text);
    let huffman_tree = build_huffman_tree(&freq_map);
    let mut codebook = HashMap::new();
    build_codebook(&huffman_tree, String::new(), &mut codebook);

    c.bench_function("huffman_encoding", |b| {
        b.iter(|| {
            let encoded = encode(black_box(text), &codebook);
            black_box(encoded);
        })
    });
}

fn huffman_decoding_benchmark(c: &mut Criterion) {
    let text = "This is a sample text for benchmarking the Huffman coding implementation.";
    let freq_map = build_frequency_map(text);
    let huffman_tree = build_huffman_tree(&freq_map);
    let mut codebook = HashMap::new();
    build_codebook(&huffman_tree, String::new(), &mut codebook);
    let encoded = encode(text, &codebook);

    c.bench_function("huffman_decoding", |b| {
        b.iter(|| {
            let decoded = decode(black_box(&encoded), &huffman_tree);
            black_box(decoded);
        })
    });
}

criterion_group!(benches, huffman_encoding_benchmark, huffman_decoding_benchmark);
criterion_main!(benches);