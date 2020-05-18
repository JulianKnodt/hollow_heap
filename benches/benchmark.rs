use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hollow_heap::hollow_heap::HollowHeap;
use std::collections::BinaryHeap;

pub fn benchmark(c: &mut Criterion) {
  let mut hh = HollowHeap::new();
  c.bench_function("push", |b| b.iter(|| {
    hh.push(black_box(1), ());
  }));
  let mut bh = BinaryHeap::new();
  c.bench_function("bh push", |b| b.iter(|| {
    bh.push(black_box(1));
  }));
}


criterion_group!(hollow_heap, benchmark);
criterion_main!(hollow_heap);
