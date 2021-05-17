use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use prefix_checkers::*;
use std::path::PathBuf;

pub fn make_vec_from_file(filename: &PathBuf) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        word_list.push(l);
    }
    word_list
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RemovePrefixWords");
    let unsafe_word_list =
        make_vec_from_file(&PathBuf::from("./benches/agile_words_first_500.txt"));

    // group.sample_size(10);
    group.bench_function("Using regular Vectors", |b| {
        b.iter(|| remove_prefix_words_vectors(&unsafe_word_list))
    });

    group.bench_function("Using fst Sets", |b| {
        b.iter(|| remove_prefix_words_fst_sets(&unsafe_word_list))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
