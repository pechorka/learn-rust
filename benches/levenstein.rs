use criterion::{Criterion, black_box, criterion_group, criterion_main};
use learn_rust::{lev_idiomatic, lev_matrix, lev_naive};

fn bench_implementations(c: &mut Criterion) {
    let mut group = c.benchmark_group("levenshtein");
    let cases = [
        ("short", "kitten", "sitting"),
        ("medium", "Saturday", "Sunday"),
        ("unicode", "γλώσσα", "glossa"),
    ];

    for (label, a, b) in cases {
        group.bench_with_input(format!("naive/{label}"), &(a, b), |bencher, (a, b)| {
            bencher.iter(|| lev_naive(black_box(a), black_box(b)));
        });

        group.bench_with_input(format!("idiomatic/{label}"), &(a, b), |bencher, (a, b)| {
            bencher.iter(|| lev_idiomatic(black_box(a), black_box(b)));
        });

        group.bench_with_input(format!("matrix/{label}"), &(a, b), |bencher, (a, b)| {
            bencher.iter(|| lev_matrix(black_box(a), black_box(b)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_implementations);
criterion_main!(benches);
