use criterion::{black_box, criterion_group, criterion_main, Criterion};

use nimble::codegen::Codegen;
use nimble::parser::Parser;
use nimble::pipeline::PipelineConfig;
use nimble::typechecker::TypeChecker;

/// A moderately complex Nimble program for benchmarking.
const BENCH_PROGRAM: &str = r#"
fn fib(n: Int) -> Int:
    if n <= 1:
        return n
    else:
        return fib(n - 1) + fib(n - 2)

fn factorial(n: Int) -> Int:
    var result = 1
    var i = 1
    while i <= n:
        result = result * i
        i = i + 1
    return result

fn main() -> Int:
    var sum = 0
    var i = 1
    while i < 100:
        if i % 3 == 0 && i % 5 == 0:
            sum = sum + i
        elif i % 3 == 0:
            sum = sum + i
        i = i + 1
    return sum
"#;

fn bench_lexer(c: &mut Criterion) {
    c.bench_function("lexer", |b| {
        b.iter(|| {
            let lexer = nimble::Lexer::new(black_box(BENCH_PROGRAM));
            let count = lexer.count();
            black_box(count);
        });
    });
}

fn bench_parser(c: &mut Criterion) {
    c.bench_function("parser", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(BENCH_PROGRAM)).unwrap();
            let prog = parser.parse().unwrap();
            black_box(prog);
        });
    });
}

fn bench_typechecker(c: &mut Criterion) {
    let prog = Parser::new(BENCH_PROGRAM).unwrap().parse().unwrap();
    c.bench_function("typechecker", |b| {
        b.iter(|| {
            let mut tc = TypeChecker::new(black_box(BENCH_PROGRAM));
            let env = tc.check_program(black_box(&prog)).unwrap();
            black_box(env);
        });
    });
}

fn bench_codegen(c: &mut Criterion) {
    let prog = Parser::new(BENCH_PROGRAM).unwrap().parse().unwrap();
    let env = TypeChecker::new(BENCH_PROGRAM)
        .check_program(&prog)
        .unwrap();
    c.bench_function("codegen", |b| {
        b.iter(|| {
            let mut cg = Codegen::new();
            cg.generate(black_box(&prog), black_box(&env)).unwrap();
            let ir = cg.into_ir();
            black_box(ir);
        });
    });
}

fn bench_full_pipeline(c: &mut Criterion) {
    c.bench_function("full_pipeline", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(BENCH_PROGRAM)).unwrap();
            let prog = parser.parse().unwrap();
            let mut tc = TypeChecker::new(black_box(BENCH_PROGRAM));
            let env = tc.check_program(&prog).unwrap();
            let mut cg = Codegen::new();
            cg.generate(&prog, &env).unwrap();
            let ir = cg.into_ir();
            black_box(ir);
        });
    });
}

fn bench_pipeline_config(c: &mut Criterion) {
    let config = PipelineConfig::default();
    c.bench_function("pipeline_config_clang_args", |b| {
        b.iter(|| {
            let args = config.to_clang_args();
            black_box(args);
        });
    });
}

criterion_group!(
    benches,
    bench_lexer,
    bench_parser,
    bench_typechecker,
    bench_codegen,
    bench_full_pipeline,
    bench_pipeline_config,
);
criterion_main!(benches);
