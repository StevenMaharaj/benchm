use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benchm::parsers::serde_parse;
// mod crate::parsers;

fn criterion_benchmark(c: &mut Criterion) {
    let file_path = "data.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = black_box(contents.lines());
    c.bench_function("serde_parsing", |b| b.iter(|| serde_parse(lines.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
