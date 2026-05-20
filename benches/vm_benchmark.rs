use criterion::{criterion_group, criterion_main, Criterion};
use nimble::compiler::Compiler;
use nimble::vm::VM;
use std::sync::Arc;

fn bench_vm(c: &mut Criterion) {
    let mut compiler = Compiler::new("bench".into());
    // Simple arithmetic loop: sum 0 to 1000
    let source = "
        let sum = 0
        for i in 0..1000 {
            sum = sum + i
        }
        out(sum)
    ";
    // Manually parse/compile logic would go here
    // For now, assume a pre-compiled chunk
    let chunk = compiler.compile_stmts(&vec![]); // Simplified for example
    
    c.bench_function("vm_loop_1000", |b| {
        b.iter(|| {
            let mut vm = VM::new();
            let _ = vm.run(Arc::clone(&chunk));
        })
    });
}

criterion_group!(benches, bench_vm);
criterion_main!(benches);
