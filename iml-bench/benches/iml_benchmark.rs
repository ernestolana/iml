
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use core::{Arena, Node, NodeType};
use checker::check_arena;

fn task_a_fibonacci() -> String {
    let arena = Arena {
        nodes: vec![
            Node { node_type: NodeType::Num(0), children: vec![], rationale: None, ownership: None },
            Node { node_type: NodeType::Num(1), children: vec![], rationale: None, ownership: None },
            Node { node_type: NodeType::Add, children: vec![0, 1], rationale: None, ownership: None },
        ]
    };
    serde_json::to_string(&arena).unwrap()
}

fn task_b_memory() -> String {
    let arena = Arena {
        nodes: vec![
            Node { node_type: NodeType::Alloc, children: vec![], rationale: None, ownership: Some("res".to_string()) },
            Node { node_type: NodeType::Drop, children: vec![0], rationale: None, ownership: None },
        ]
    };
    serde_json::to_string(&arena).unwrap()
}

fn task_c_tensor() -> String {
    let arena = Arena {
        nodes: vec![
            Node { node_type: NodeType::AlgebraicMatrix(vec![1.0, 2.0, 3.0, 4.0], 2, 2), children: vec![], rationale: None, ownership: None },
            Node { node_type: NodeType::QuantumState(vec![0.5, 0.5]), children: vec![], rationale: None, ownership: None },
        ]
    };
    serde_json::to_string(&arena).unwrap()
}

fn benchmark_ingestion(c: &mut Criterion) {
    let json_a = task_a_fibonacci();
    let json_b = task_b_memory();
    let json_c = task_c_tensor();

    c.bench_function("parse_task_a", |b| b.iter(|| {
        let arena: Arena = serde_json::from_str(black_box(&json_a)).unwrap();
        black_box(arena);
    }));

    c.bench_function("parse_task_b", |b| b.iter(|| {
        let arena: Arena = serde_json::from_str(black_box(&json_b)).unwrap();
        black_box(arena);
    }));

    c.bench_function("parse_task_c", |b| b.iter(|| {
        let arena: Arena = serde_json::from_str(black_box(&json_c)).unwrap();
        black_box(arena);
    }));
}

fn benchmark_validation(c: &mut Criterion) {
    let arena_a: Arena = serde_json::from_str(&task_a_fibonacci()).unwrap();
    let arena_b: Arena = serde_json::from_str(&task_b_memory()).unwrap();
    
    c.bench_function("validate_task_a", |b| b.iter(|| {
        check_arena(black_box(&arena_a)).unwrap();
    }));
    c.bench_function("validate_task_b", |b| b.iter(|| {
        check_arena(black_box(&arena_b)).unwrap();
    }));
}

criterion_group!(benches, benchmark_ingestion, benchmark_validation);
criterion_main!(benches);

