use algorithms::numbers::fibonacci::{
    binet_formula,
    bottom_up,
    bottom_up_space_optimized,
    fast_doubling_recursive,
    lookup_table,
    matrix_exponentiation_optimized,
    matrix_exponentiation_recursive,
    top_down_memoized,
    // recursive,
};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    let i = 20_usize;
    group.bench_with_input(BenchmarkId::new("binet_formula", i), &i, |b, i| {
        b.iter(|| binet_formula::fibonacci(*i))
    });
    group.bench_with_input(BenchmarkId::new("bottom_up", i), &i, |b, i| {
        b.iter(|| bottom_up::fibonacci(*i))
    });
    group.bench_with_input(
        BenchmarkId::new("bottom_up_space_optimized", i),
        &i,
        |b, i| b.iter(|| bottom_up_space_optimized::fibonacci(*i)),
    );
    group.bench_with_input(
        BenchmarkId::new("fast_doubling_recursive", i),
        &i,
        |b, i| b.iter(|| fast_doubling_recursive::fibonacci(*i)),
    );
    group.bench_with_input(BenchmarkId::new("lookup_table", i), &i, |b, i| {
        b.iter(|| lookup_table::fibonacci(*i))
    });
    group.bench_with_input(
        BenchmarkId::new("matrix_exponentiation_optimized", i),
        &i,
        |b, i| b.iter(|| matrix_exponentiation_optimized::fibonacci(*i)),
    );
    group.bench_with_input(
        BenchmarkId::new("matrix_exponentiation_recursive", i),
        &i,
        |b, i| b.iter(|| matrix_exponentiation_recursive::fibonacci(*i)),
    );
    // group.bench_with_input(BenchmarkId::new("recursive", i), &i, |b, i| b.iter(|| recursive::fibonacci(*i)));
    group.bench_with_input(BenchmarkId::new("top_down_memoized", i), &i, |b, i| {
        b.iter(|| top_down_memoized::fibonacci(*i))
    });
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
