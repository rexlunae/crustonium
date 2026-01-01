// Benchmark: Rust vs C++ Performance Comparison
//
// Phase 1.1: Performance benchmarking
// Compares equivalent functionality implemented in both Rust and C++

use cargo_cpp_integration::{RustComponent, ffi::{create_legacy_component, process_via_cpp}};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_rust_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_processing");
    
    for size in [64, 256, 1024, 4096].iter() {
        let input: Vec<u8> = (0u8..=255).cycle().take(*size).collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            let mut component = RustComponent::new();
            b.iter(|| {
                component.process_data(black_box(&input)).unwrap()
            });
        });
    }
    
    group.finish();
}

fn benchmark_cpp_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpp_processing");
    
    for size in [64, 256, 1024, 4096].iter() {
        let input: Vec<u8> = (0u8..=255).cycle().take(*size).collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            let component = create_legacy_component();
            b.iter(|| {
                let mut output = Vec::new();
                process_via_cpp(&component, black_box(&input), &mut output)
            });
        });
    }
    
    group.finish();
}

fn benchmark_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_vs_cpp");
    let size = 1024;
    let input: Vec<u8> = (0u8..=255).cycle().take(size).collect();
    
    group.bench_function("rust", |b| {
        let mut component = RustComponent::new();
        b.iter(|| {
            component.process_data(black_box(&input)).unwrap()
        });
    });
    
    group.bench_function("cpp", |b| {
        let component = create_legacy_component();
        b.iter(|| {
            let mut output = Vec::new();
            process_via_cpp(&component, black_box(&input), &mut output)
        });
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_rust_processing, benchmark_cpp_processing, benchmark_comparison);
criterion_main!(benches);
