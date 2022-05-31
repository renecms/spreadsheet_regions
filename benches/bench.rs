use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use spreadsheet_regions::{get_values_from_regions, getting_data_directly_from_cell, sheet::Sheet};

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Spreadsheet");
    group.sample_size(10);
    for i in (1000..10000).step_by(1000) {
        group.bench_with_input(BenchmarkId::new("values from regions", i), &i, |b, i| {
            b.iter(|| {
                let sheet = Sheet::new(black_box(*i), black_box(10));
                get_values_from_regions(&sheet.borrow());
            })
        });
        group.bench_with_input(
            BenchmarkId::new("values from cells with preprocessing", i),
            &i,
            |b, i| {
                b.iter(|| {
                    let sheet = Sheet::new(black_box(*i), black_box(10));
                    getting_data_directly_from_cell(&sheet.borrow());
                })
            },
        );
    }
    group.finish();
}
criterion_group!(benches, benchmarks);
criterion_main!(benches);
